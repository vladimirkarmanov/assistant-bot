use sqlx::{Pool, Sqlite};

pub async fn add_class(
    db: &Pool<Sqlite>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(db)
        .await?;

    Ok(row.0.to_string())
}
