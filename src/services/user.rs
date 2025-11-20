use std::sync::Arc;

use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(FromRow)]
pub struct User {
    pub user_id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
}

pub async fn does_user_exist(db: Arc<Pool<Sqlite>>, telegram_id: i64) -> anyhow::Result<bool> {
    let user: Option<(i64,)> = sqlx::query_as("select user_id from user where telegram_id = ?")
        .bind(telegram_id)
        .fetch_optional(db.as_ref())
        .await?;

    Ok(user.is_some())
}

pub async fn get_user_by_telegram_id(
    db: Arc<Pool<Sqlite>>,
    telegram_id: i64,
) -> anyhow::Result<Option<User>> {
    let user: Option<User> = sqlx::query_as::<_, User>(
        "select user_id, telegram_id, username from user where telegram_id = ?",
    )
    .bind(telegram_id)
    .fetch_optional(db.as_ref())
    .await?;

    Ok(user)
}

pub async fn add_user(
    db: Arc<Pool<Sqlite>>,
    telegram_id: i64,
    username: &str,
) -> anyhow::Result<()> {
    if !does_user_exist(Arc::clone(&db), telegram_id).await? {
        sqlx::query("insert into user (telegram_id, username) values (?, ?)")
            .bind(telegram_id)
            .bind(username)
            .execute(db.as_ref())
            .await?;
    }
    Ok(())
}
