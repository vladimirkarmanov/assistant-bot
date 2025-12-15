use std::ops::DerefMut;

use sqlx::{SqliteConnection, prelude::FromRow};

#[derive(FromRow)]
pub struct User {
    pub username: Option<String>,
    pub user_id: i64,
    pub telegram_id: i64,
}

pub struct UserRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub async fn exists(&mut self, telegram_id: i64) -> anyhow::Result<bool> {
        let user: Option<(i64,)> = sqlx::query_as("select user_id from user where telegram_id = ?")
            .bind(telegram_id)
            .fetch_optional(self.conn.deref_mut())
            .await?;
        Ok(user.is_some())
    }

    pub async fn create(&mut self, telegram_id: i64, username: &str) -> anyhow::Result<i64> {
        let result = sqlx::query("insert into user (telegram_id, username) values (?, ?)")
            .bind(telegram_id)
            .bind(username)
            .execute(self.conn.deref_mut())
            .await?;

        let user_id = result.last_insert_rowid();
        Ok(user_id)
    }

    pub async fn get_user_by_telegram_id(
        &mut self,
        telegram_id: i64,
    ) -> anyhow::Result<Option<User>> {
        let user: Option<User> = sqlx::query_as::<_, User>(
            "select user_id, telegram_id, username from user where telegram_id = ?",
        )
        .bind(telegram_id)
        .fetch_optional(self.conn.deref_mut())
        .await?;

        Ok(user)
    }
}
