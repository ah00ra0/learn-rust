//! مقدمه‌ای بر sqlx با SQLite درون‌حافظه.

use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};

/// یک pool با پایگاه‌داده‌ی in-memory و ساخت جدول.
///
/// برای in-memory باید تعداد اتصال ۱ باشد: هر اتصال SQLite درون‌حافظه
/// پایگاه‌داده‌ی جدا و خالی خودش را دارد.
pub async fn connect_in_memory() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
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

pub async fn titles_by_author(pool: &SqlitePool, author: &str) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT title FROM books WHERE author = ? ORDER BY id")
        .bind(author)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|r| r.get::<String, _>("title")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn insert_and_query() {
        let pool = connect_in_memory().await.unwrap();
        let id = insert_book(&pool, "شاهنامه", "فردوسی").await.unwrap();
        assert_eq!(id, 1);
        insert_book(&pool, "گلستان", "سعدی").await.unwrap();
        let titles = titles_by_author(&pool, "فردوسی").await.unwrap();
        assert_eq!(titles, vec!["شاهنامه"]);
    }

    #[tokio::test]
    async fn bound_parameters_prevent_injection() {
        let pool = connect_in_memory().await.unwrap();
        insert_book(&pool, "a", "x").await.unwrap();
        let titles = titles_by_author(&pool, "x' OR '1'='1").await.unwrap();
        assert!(titles.is_empty());
    }
}
