use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
}

pub async fn add_user(
    db: Pool<Sqlite>,
    telegram_id: i64,
    username: &str,
) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
    let result = sqlx::query("insert into user (telegram_id, username) values (?, ?)")
        .bind(telegram_id)
        .bind(username)
        .execute(&db)
        .await?;

    let user_id = result.last_insert_rowid();

    let user = sqlx::query_as::<_, User>(
        "select user_id, telegram_id, username from user where user_id = ?",
    )
    .bind(user_id)
    .fetch_one(&db)
    .await?;

    Ok(user)
}
