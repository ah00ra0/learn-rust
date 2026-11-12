# ساخت REST API با Axum

وب‌سرور یکی از محبوب‌ترین کاربردهای Rust است. در این فصل با **Axum** (فریم‌ورک وب مبتنی بر Tokio و Tower، ساخته‌ی تیم Tokio) یک REST API کامل برای مدیریت «کتاب‌ها» می‌سازیم: ساخت، خواندن، ویرایش، حذف، اعتبارسنجی، خطای مناسب، middleware، تست خودکار و خاموشی تمیز.

> **پیش‌نیاز:** فصل‌های [Concurrency](21-concurrency.md)، [مدیریت خطا](14-errors.md)، [Trait](16-traits.md) و [Async](26-async.md). کد کامل و تست‌شده در [`projects/axum-api`](https://github.com/ah00ra0/learn-rust/tree/main/projects/axum-api) است (با Axum نسخه‌ی 0.8 ساخته و تست شده؛ API این crate بین نسخه‌های major تغییر می‌کند، پس مستندات نسخه‌ی خودتان را هم نگاه کنید).

بلوک‌های کد این فصل `ignore` هستند چون به crate های خارجی نیاز دارند، ولی همه‌ی آن‌ها بخشی از پروژه‌ی بالاست که با `cargo test` (۱۲ تست) و `cargo clippy -- -D warnings` پاس می‌شود.

## معماری در یک نگاه

```text
درخواست HTTP
   │
   ▼
Middleware ها (Trace، CORS، ...)       ← لایه‌های Tower
   │
   ▼
Router: مسیر + متد ──▶ Handler (تابع async)
                         │  ورودی‌ها: Extractor ها (Path, Query, Json, State)
                         ▼
                       خروجی: هر چیزی که IntoResponse را پیاده کند
```

سه مفهوم کلیدی:

- **Handler**: یک `async fn` که از Extractor ها ورودی می‌گیرد و چیزی که به پاسخ قابل تبدیل است برمی‌گرداند.
- **Extractor**: نوعی که `FromRequest` / `FromRequestParts` را پیاده کرده و بخشی از درخواست را استخراج می‌کند.
- **IntoResponse**: trait ی که هر مقدار را به پاسخ HTTP تبدیل می‌کند. `&str`، `String`، `StatusCode`، `Json<T>`، tuple ها و `Result<T, E>` همگی آن را دارند.

## وابستگی‌ها

```toml
[dependencies]
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.7", features = ["trace", "cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
```

## سلام دنیا

```rust,ignore
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "سلام از Axum" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

با `curl localhost:3000` پاسخ می‌گیرید. `#[tokio::main]` همان runtime فصل Async را راه می‌اندازد.

## مدل‌های داده

```rust,ignore
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub year: u16,
}

/// بدنه‌ی ساخت کتاب: بدون id (سرور آن را می‌دهد)
#[derive(Debug, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: u16,
}

/// بدنه‌ی PATCH: هر فیلد اختیاری
#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<u16>,
}

/// پارامترهای query: /books?author=...&limit=...
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub author: Option<String>,
    pub limit: Option<usize>,
}
```

جدا کردن `Book` از `NewBook` الگوی رایج و ایمنی است: کلاینت نمی‌تواند `id` را خودش تعیین کند.

## حالت مشترک (State)

handler ها همزمان روی چند thread اجرا می‌شوند، پس داده‌ی مشترک باید thread-safe باشد (فصل [Concurrency](21-concurrency.md)): `Arc` برای مالکیت مشترک، `RwLock` برای دسترسی.

```rust,ignore
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, atomic::{AtomicU64, Ordering}},
};

#[derive(Clone, Default)]
pub struct AppState {
    books: Arc<RwLock<HashMap<u64, Book>>>,
    next_id: Arc<AtomicU64>,
}
```

- `Clone` ارزان است چون فقط شمارنده‌ی `Arc` زیاد می‌شود. Axum برای هر درخواست state را clone می‌کند.
- `AtomicU64` برای تولید id بدون قفل.

> **هشدار مهم:** در handler های async **قفل `std::sync::RwLock` را هرگز در طول یک `.await` نگه ندارید**؛ ممکن است deadlock یا خطای `Send` بگیرید. در این مثال هیچ `.await` ی در حین نگه‌داشتن قفل نیست. اگر لازم دارید، از `tokio::sync::RwLock` یا ساختار دیگر (مثل کانال) استفاده کنید ([Async در عمق](async-advanced.md)).

## خطا: تبدیل به پاسخ HTTP

به‌جای برگرداندن خطای خام، enum خطای برنامه را تعریف می‌کنیم و `IntoResponse` را برایش پیاده می‌کنیم. این دقیقاً همان الگوی [مدیریت خطا](14-errors.md) است، فقط خروجی‌اش پاسخ HTTP است:

```rust,ignore
use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};

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
```

حالا handler ها می‌توانند `Result<T, ApiError>` برگردانند و عملگر `?` همه‌چیز را ساده می‌کند.

## Handler ها و Extractor ها

```rust,ignore
use axum::extract::{Path, Query, State};

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
    let book = Book { id, title: new.title, author: new.author, year: new.year };
    state.books.write().unwrap().insert(id, book.clone());
    Ok((StatusCode::CREATED, Json(book)))
}

async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Book>, ApiError> {
    let books = state.books.read().unwrap();
    books.get(&id).cloned().map(Json).ok_or(ApiError::NotFound(id))
}
```

نکات مهم:

۱. **ترتیب Extractor ها:** extractor ی که **بدنه‌ی درخواست** را مصرف می‌کند (`Json`, `String`, `Bytes`) باید **آخرین** پارامتر باشد، چون بدنه فقط یک بار قابل خواندن است.
۲. اگر extractor شکست بخورد (JSON خراب، `id` غیرعددی)، Axum بدون اجرای handler خودش پاسخ ۴۰۰ یا ۴۲۲ می‌دهد.
۳. `Json(new): Json<NewBook>` از **destructuring در پارامتر** (فصل [الگوها](22-patterns.md)) استفاده می‌کند.
۴. خروجی `(StatusCode, Json<Book>)` tuple ای است که خودش `IntoResponse` است.
۵. عبارت `is_none_or` در فیلتر: «اگر پارامتر author نیامده یا برابر است». (برای Rust 1.82+)

### ویرایش و حذف

```rust,ignore
async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(patch): Json<UpdateBook>,
) -> Result<Json<Book>, ApiError> {
    let mut books = state.books.write().unwrap();
    let book = books.get_mut(&id).ok_or(ApiError::NotFound(id))?;
    let title = patch.title.unwrap_or_else(|| book.title.clone());
    let author = patch.author.unwrap_or_else(|| book.author.clone());
    validate(&title, &author)?;          // قبل از تغییر، اعتبارسنجی
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
```

دقت کنید `update_book` اول مقادیر جدید را در متغیر محلی می‌سازد، اعتبارسنجی می‌کند و **بعد** تغییر می‌دهد؛ در غیر این صورت درخواست نامعتبر می‌توانست کتاب را نیمه‌به‌روز رها کند.

## Router و Middleware

```rust,ignore
use axum::{Router, routing::get};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/books", get(list_books).post(create_book))
        .route("/books/{id}", get(get_book).patch(update_book).delete(delete_book))
        .layer(TraceLayer::new_for_http())     // لاگ هر درخواست
        .layer(CorsLayer::permissive())        // CORS (برای توسعه؛ در تولید محدودش کنید)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
```

- در Axum 0.8 پارامتر مسیر با `{id}` نوشته می‌شود (نسخه‌های قدیمی‌تر `:id` داشتند).
- هر `.layer(...)` یک middleware از اکوسیستم **Tower** است. چون Axum روی Tower ساخته شده، همه‌ی middleware های Tower (timeout، rate-limit، compression، auth) قابل استفاده‌اند.
- `CorsLayer::permissive()` همه‌ی origin ها را مجاز می‌کند؛ فقط برای توسعه.
- تابع `app()` را از `main` جدا کردیم تا **بدون راه‌اندازی سرور واقعی قابل تست** باشد.

## اجرا و امتحان

```rust,ignore
use axum_api::{AppState, app};
use tokio::{net::TcpListener, signal};

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("نصب handler برای Ctrl+C شکست خورد");
    tracing::info!("سیگنال خاموشی دریافت شد؛ درخواست‌های در حال انجام تمام می‌شوند");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("گوش دادن روی {}", listener.local_addr().unwrap());

    axum::serve(listener, app(AppState::default()))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
```

`with_graceful_shutdown` سرور را وقتی Ctrl+C آمد، پس از اتمام درخواست‌های در حال اجرا می‌بندد (بدون قطع وسط پاسخ).

اجرا کنید:

```bash
cd projects/axum-api
cargo run
```

و در ترمینال دیگر (خروجی‌های زیر از اجرای واقعی همین پروژه گرفته شده‌اند):

```bash
curl -X POST localhost:3000/books -H 'content-type: application/json' \
     -d '{"title":"شاهنامه","author":"فردوسی","year":1010}'
```

```text
{"id":1,"title":"شاهنامه","author":"فردوسی","year":1010}
```

```bash
curl -X POST localhost:3000/books -H 'content-type: application/json' \
     -d '{"title":"گلستان","author":"سعدی","year":1258}'
curl 'localhost:3000/books?author=سعدی'
```

```text
[{"id":2,"title":"گلستان","author":"سعدی","year":1258}]
```

```bash
curl -i localhost:3000/books/99        # ۴۰۴
```

```text
{"error":"کتاب 99 پیدا نشد"}
```

```bash
curl -X PATCH localhost:3000/books/1 -H 'content-type: application/json' -d '{"year":1011}'
```

```text
{"id":1,"title":"شاهنامه","author":"فردوسی","year":1011}
```

حذف کتاب کد ۲۰۴ برمی‌گرداند (بدون بدنه) و درخواست با عنوان خالی کد ۴۲۲ با `{"error":"عنوان نباید خالی باشد"}`.

## تست بدون سرور واقعی

چون `Router` در واقع یک Tower `Service` است، می‌توان درخواست را مستقیم به آن داد و پاسخ را گرفت؛ **بدون شبکه و بدون پورت**. تست سریع و پایدار:

```rust,ignore
use axum::{body::Body, http::{Method, Request, header}};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt; // برای oneshot

async fn call(app: &Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(v) => {
            builder = builder.header(header::CONTENT_TYPE, "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let response = app.clone().oneshot(builder.body(body).unwrap()).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
}

#[tokio::test]
async fn create_then_get() {
    let app = app(AppState::default());
    let (status, created) = call(
        &app, Method::POST, "/books",
        Some(json!({"title": "شاهنامه", "author": "فردوسی", "year": 1010})),
    ).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["id"], 1);

    let (status, fetched) = call(&app, Method::GET, "/books/1", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched, created);
}
```

نکات:

- `app.clone()`: `oneshot` سرویس را مصرف می‌کند؛ clone ارزان است و **state مشترک می‌ماند** (چون `Arc`)، پس چند درخواست پشت‌سرهم اثر هم را می‌بینند.
- پروژه تست‌های حالت خطا را هم دارد: JSON خراب (۴۰۰)، `id` غیرعددی (۴۰۰)، عنوان خالی (۴۲۲)، `PATCH` جزئی، حذف دوباره (۴۰۴) و هدر CORS.
- برای تست‌های یکپارچه با سرور واقعی از `TcpListener::bind("127.0.0.1:0")` (پورت تصادفی) و یک کلاینت HTTP (مثل `reqwest`) استفاده کنید.

## مقدمه‌ای بر پایگاه‌داده با sqlx

حالت حافظه‌ای فقط برای آموزش است؛ داده‌ی واقعی در پایگاه‌داده می‌ماند. **sqlx** یک کتابخانه‌ی async SQL است. پروژه یک ماژول کوچک `db.rs` دارد (با SQLite درون‌حافظه تا نصب سرور لازم نباشد):

```rust,ignore
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect_in_memory() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)   // هر اتصال SQLite in-memory پایگاه‌داده‌ی جدای خودش را دارد
        .connect("sqlite::memory:")
        .await?;
    sqlx::query(
        "CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub async fn insert_book(pool: &SqlitePool, title: &str, author: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO books (title, author) VALUES (?, ?)")
        .bind(title)
        .bind(author)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}
```

نکته‌ی امنیتی مهم: مقادیر را **همیشه با `.bind()`** بدهید، نه با الحاق رشته در SQL. تست پروژه نشان می‌دهد ورودی مخرب `x' OR '1'='1` به‌عنوان یک مقدار ساده‌ی متنی رفتار می‌کند و چیزی برنمی‌گرداند:

```rust,ignore
#[tokio::test]
async fn bound_parameters_prevent_injection() {
    let pool = connect_in_memory().await.unwrap();
    insert_book(&pool, "a", "x").await.unwrap();
    let titles = titles_by_author(&pool, "x' OR '1'='1").await.unwrap();
    assert!(titles.is_empty());
}
```

برای اتصال حقیقی به PostgreSQL یا MySQL، رشته‌ی اتصال و feature مربوط (`postgres`, `mysql`) را عوض کنید. sqlx همچنین ماکروی `query!` دارد که SQL را **در زمان کامپایل** با پایگاه‌داده‌ی واقعی بررسی می‌کند (به `DATABASE_URL` در زمان build نیاز دارد).

## نکات تولید (Production)

- **محدودیت‌ها:** `tower-http` لایه‌های `TimeoutLayer`، `RequestBodyLimitLayer` و `CompressionLayer` دارد. بدون محدودیت اندازه‌ی بدنه، کلاینت می‌تواند حافظه‌ی سرور را پر کند.
- **لاگ ساخت‌یافته:** `tracing` + `TraceLayer` (فصل [خطا و لاگ](errors-logging-libs.md)).
- **احراز هویت:** به‌صورت extractor اختصاصی (`FromRequestParts`) یا middleware بنویسید.
- **اعتبارسنجی:** تابع `validate` ساده است؛ برای قواعد پیچیده از crate ی `validator` استفاده کنید.
- **پیکربندی:** پورت و رشته‌ی اتصال را از متغیر محیطی بخوانید ([فصل ورودی/خروجی](formatting-and-io.md)).
- **مقیاس:** سرور Tokio به‌طور پیش‌فرض از همه‌ی هسته‌ها استفاده می‌کند. کار CPU-سنگین را با `spawn_blocking` ببرید.
- **بیلد انتشار:** `cargo build --release` و فصل [عملکرد](performance.md).

## جمع‌بندی

| مفهوم | ابزار |
|---|---|
| مسیریابی | `Router::route`، `.get/.post/...` |
| ورودی | Extractor ها: `Path`, `Query`, `Json`, `State` |
| خروجی | `IntoResponse`، `Json`، `StatusCode`، tuple |
| خطا | enum + `impl IntoResponse` + `?` |
| حالت مشترک | `Arc<RwLock<...>>` یا `Arc<Mutex<...>>` در `State` |
| میان‌افزار | لایه‌های Tower (`.layer`) |
| تست | `ServiceExt::oneshot` بدون سرور |
| خاموشی | `with_graceful_shutdown` |
| دیتابیس | `sqlx` با `bind` |

## تمرین

1. فیلد `isbn: Option<String>` به `Book` اضافه کنید و تست‌ها را به‌روز کنید.
2. endpoint ی `GET /books/search?q=...` بنویسید که در عنوان جستجو کند (بدون حساسیت به حروف).
3. با `RequestBodyLimitLayer` حداکثر اندازه‌ی بدنه را ۱ کیلوبایت کنید و با یک تست نشان دهید درخواست بزرگ رد می‌شود.
4. حافظه‌ی `HashMap` را با sqlx و SQLite فایل‌محور (`sqlite://books.db?mode=rwc`) جایگزین کنید.
5. یک extractor اختصاصی بنویسید که هدر `x-api-key` را بررسی کند و در غیر این صورت ۴۰۱ بدهد.
