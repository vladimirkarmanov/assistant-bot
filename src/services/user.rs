use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
}

pub async fn does_user_exist(
    db: &Pool<Sqlite>,
    telegram_id: i64,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let user: Option<(i64,)> = sqlx::query_as("select user_id from user where telegram_id = ?")
        .bind(telegram_id)
        .fetch_optional(db)
        .await?;

    Ok(user.is_some())
}

pub async fn add_user(
    db: &Pool<Sqlite>,
    telegram_id: i64,
    username: &str,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    if does_user_exist(db, telegram_id).await? {
        return Err("User already exists".into());
    }
    let result = sqlx::query("insert into user (telegram_id, username) values (?, ?)")
        .bind(telegram_id)
        .bind(username)
        .execute(db)
        .await?;

    let user_id = result.last_insert_rowid();
    Ok(user_id)
}
