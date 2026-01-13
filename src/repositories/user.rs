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

#[cfg(test)]
mod tests {
    use super::{User, UserRepository};
    use sqlx::Row;

    use crate::test_utils;

    #[tokio::test]
    async fn test_exists_false_then_true() -> anyhow::Result<()> {
        let pool = test_utils::setup_db().await;
        let mut conn = pool.acquire().await?;
        let mut repo = UserRepository::new(&mut conn);

        let telegram_id = 1111_i64;

        assert!(!repo.exists(telegram_id).await?);

        repo.create(telegram_id, "user1").await?;
        assert!(repo.exists(telegram_id).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_returns_id_and_persists() -> anyhow::Result<()> {
        let pool = test_utils::setup_db().await;
        let mut conn = pool.acquire().await?;
        let mut repo = UserRepository::new(&mut conn);

        let telegram_id = 2222_i64;
        let username = "user2";

        let user_id = repo.create(telegram_id, username).await?;
        assert!(user_id > 0);

        let row =
            sqlx::query("SELECT user_id, telegram_id, username FROM user WHERE telegram_id = ?")
                .bind(telegram_id)
                .fetch_one(conn.as_mut())
                .await?;

        let stored_user_id: i64 = row.get("user_id");
        let stored_telegram_id: i64 = row.get("telegram_id");
        let stored_username: String = row.get("username");

        assert_eq!(stored_user_id, user_id);
        assert_eq!(stored_telegram_id, telegram_id);
        assert_eq!(stored_username, username);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_by_telegram_id() -> anyhow::Result<()> {
        let pool = test_utils::setup_db().await;
        let mut conn = pool.acquire().await?;
        let mut repo = UserRepository::new(&mut conn);

        let telegram_id = 3333_i64;
        let username = "user3";

        let before = repo.get_user_by_telegram_id(telegram_id).await?;
        assert!(before.is_none());

        let created_id = repo.create(telegram_id, username).await?;
        let fetched = repo.get_user_by_telegram_id(telegram_id).await?;
        let user: User = fetched.expect("user should exist");

        assert_eq!(user.user_id, created_id);
        assert_eq!(user.telegram_id, telegram_id);
        assert_eq!(user.username.as_deref(), Some(username));

        Ok(())
    }
}
