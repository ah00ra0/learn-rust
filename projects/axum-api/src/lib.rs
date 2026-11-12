//! API ساده‌ی «کتاب‌ها» با Axum؛ همراه فصل «ساخت REST API با Axum».

pub mod db;

use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// یک کتاب ذخیره‌شده.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub year: u16,
}

/// بدنه‌ی درخواست ساخت کتاب.
#[derive(Debug, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: u16,
}

/// بدنه‌ی PATCH: هر فیلد اختیاری است.
#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<u16>,
}

/// پارامترهای query برای فهرست کتاب‌ها: `?author=...&limit=...`
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub author: Option<String>,
    pub limit: Option<usize>,
}

/// حالت مشترک برنامه. `Clone` ارزان است چون فقط `Arc` کپی می‌شود.
#[derive(Clone, Default)]
pub struct AppState {
    books: Arc<RwLock<HashMap<u64, Book>>>,
    next_id: Arc<AtomicU64>,
}

/// خطاهای API که به پاسخ HTTP تبدیل می‌شوند.
#[derive(Debug)]
pub enum ApiError {
    NotFound(u64),
    Validation(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(id) => (StatusCode::NOT_FOUND, format!("کتاب {id} پیدا نشد")),
            ApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

fn validate(title: &str, author: &str) -> Result<(), ApiError> {
    if title.trim().is_empty() {
        return Err(ApiError::Validation("عنوان نباید خالی باشد".into()));
    }
    if author.trim().is_empty() {
        return Err(ApiError::Validation("نویسنده نباید خالی باشد".into()));
    }
    Ok(())
}

async fn list_books(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<Vec<Book>> {
    let books = state.books.read().unwrap();
    let mut result: Vec<Book> = books
        .values()
        .filter(|b| params.author.as_ref().is_none_or(|a| &b.author == a))
        .cloned()
        .collect();
    result.sort_by_key(|b| b.id);
    if let Some(limit) = params.limit {
        result.truncate(limit);
    }
    Json(result)
}

async fn create_book(
    State(state): State<AppState>,
    Json(new): Json<NewBook>,
) -> Result<(StatusCode, Json<Book>), ApiError> {
    validate(&new.title, &new.author)?;
    let id = state.next_id.fetch_add(1, Ordering::Relaxed) + 1;
    let book = Book {
        id,
        title: new.title,
        author: new.author,
        year: new.year,
    };
    state.books.write().unwrap().insert(id, book.clone());
    Ok((StatusCode::CREATED, Json(book)))
}

async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Book>, ApiError> {
    let books = state.books.read().unwrap();
    books
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound(id))
}

async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(patch): Json<UpdateBook>,
) -> Result<Json<Book>, ApiError> {
    let mut books = state.books.write().unwrap();
    let book = books.get_mut(&id).ok_or(ApiError::NotFound(id))?;
    let title = patch.title.unwrap_or_else(|| book.title.clone());
    let author = patch.author.unwrap_or_else(|| book.author.clone());
    validate(&title, &author)?;
    book.title = title;
    book.author = author;
    if let Some(year) = patch.year {
        book.year = year;
    }
    Ok(Json(book.clone()))
}

async fn delete_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    match state.books.write().unwrap().remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(ApiError::NotFound(id)),
    }
}

async fn health() -> &'static str {
    "ok"
}

/// ساخت Router کامل با middleware ها.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/books", get(list_books).post(create_book))
        .route(
            "/books/{id}",
            get(get_book).patch(update_book).delete(delete_book),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request, header},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    async fn call(
        app: &Router,
        method: Method,
        uri: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let mut builder = Request::builder().method(method).uri(uri);
        let body = match body {
            Some(v) => {
                builder = builder.header(header::CONTENT_TYPE, "application/json");
                Body::from(v.to_string())
            }
            None => Body::empty(),
        };
        let response = app
            .clone()
            .oneshot(builder.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, value)
    }

    fn new_app() -> Router {
        app(AppState::default())
    }

    #[tokio::test]
    async fn health_ok() {
        let response = new_app()
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&bytes[..], b"ok");
    }

    #[tokio::test]
    async fn create_then_get() {
        let app = new_app();
        let (status, created) = call(
            &app,
            Method::POST,
            "/books",
            Some(json!({"title": "شاهنامه", "author": "فردوسی", "year": 1010})),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created["id"], 1);

        let (status, fetched) = call(&app, Method::GET, "/books/1", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(fetched, created);
    }

    #[tokio::test]
    async fn get_missing_is_404() {
        let (status, body) = call(&new_app(), Method::GET, "/books/99", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body["error"].as_str().unwrap().contains("99"));
    }

    #[tokio::test]
    async fn empty_title_is_422() {
        let (status, _) = call(
            &new_app(),
            Method::POST,
            "/books",
            Some(json!({"title": "  ", "author": "x", "year": 2000})),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn malformed_json_is_rejected_by_extractor() {
        let request = Request::post("/books")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{not json"))
            .unwrap();
        let response = new_app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn invalid_path_param_is_400() {
        let (status, _) = call(&new_app(), Method::GET, "/books/abc", None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn list_filters_and_limits() {
        let app = new_app();
        for (t, a) in [("الف", "x"), ("ب", "y"), ("پ", "x")] {
            call(
                &app,
                Method::POST,
                "/books",
                Some(json!({"title": t, "author": a, "year": 2000})),
            )
            .await;
        }
        let (_, all) = call(&app, Method::GET, "/books", None).await;
        assert_eq!(all.as_array().unwrap().len(), 3);
        let (_, by_author) = call(&app, Method::GET, "/books?author=x", None).await;
        assert_eq!(by_author.as_array().unwrap().len(), 2);
        let (_, limited) = call(&app, Method::GET, "/books?author=x&limit=1", None).await;
        assert_eq!(limited.as_array().unwrap().len(), 1);
        assert_eq!(limited[0]["title"], "الف");
    }

    #[tokio::test]
    async fn patch_updates_only_given_fields() {
        let app = new_app();
        call(
            &app,
            Method::POST,
            "/books",
            Some(json!({"title": "a", "author": "b", "year": 1})),
        )
        .await;
        let (status, updated) =
            call(&app, Method::PATCH, "/books/1", Some(json!({"year": 2}))).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(updated["year"], 2);
        assert_eq!(updated["title"], "a");

        let (status, _) = call(&app, Method::PATCH, "/books/1", Some(json!({"title": ""}))).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn delete_then_404() {
        let app = new_app();
        call(
            &app,
            Method::POST,
            "/books",
            Some(json!({"title": "a", "author": "b", "year": 1})),
        )
        .await;
        let (status, _) = call(&app, Method::DELETE, "/books/1", None).await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let (status, _) = call(&app, Method::DELETE, "/books/1", None).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn cors_header_is_present() {
        let request = Request::get("/health")
            .header(header::ORIGIN, "http://example.com")
            .body(Body::empty())
            .unwrap();
        let response = new_app().oneshot(request).await.unwrap();
        assert_eq!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .unwrap(),
            "*"
        );
    }
}
