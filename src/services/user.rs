use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::uow::UnitOfWork;

pub async fn add_user(
    db_pool: Arc<Pool<Sqlite>>,
    telegram_id: i64,
    username: &str,
) -> anyhow::Result<()> {
    let mut uow = UnitOfWork::new_transactional(db_pool.as_ref()).await?;
    let mut user_repo = uow.user_repo().await?;
    if !user_repo.exists(telegram_id).await? {
        user_repo.create(telegram_id, username).await?;
        uow.commit().await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sqlx::{Pool, Row, Sqlite};

    use crate::test_utils;

    use super::add_user;

    #[tokio::test]
    async fn test_add_user_creates_when_not_exists() -> anyhow::Result<()> {
        let pool = test_utils::setup_db().await;
        let arc_pool: Arc<Pool<Sqlite>> = Arc::new(pool.clone());

        let telegram_id = 12345_i64;
        let username = "alice";

        add_user(arc_pool.clone(), telegram_id, username).await?;

        let row = sqlx::query("SELECT COUNT(*) as cnt FROM user WHERE telegram_id = ?")
            .bind(telegram_id)
            .fetch_one(&pool)
            .await?;
        let count: i64 = row.get::<i64, _>("cnt");
        assert_eq!(count, 1);

        let row = sqlx::query("SELECT username FROM user WHERE telegram_id = ?")
            .bind(telegram_id)
            .fetch_one(&pool)
            .await?;
        let stored_username: String = row.get::<String, _>("username");
        assert_eq!(stored_username, username);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_user_is_idempotent() -> anyhow::Result<()> {
        let pool = test_utils::setup_db().await;
        let arc_pool: Arc<Pool<Sqlite>> = Arc::new(pool.clone());

        let telegram_id = 67890_i64;
        let username = "bob";

        add_user(arc_pool.clone(), telegram_id, username).await?;
        add_user(arc_pool.clone(), telegram_id, username).await?;

        let row = sqlx::query("SELECT COUNT(*) as cnt FROM user WHERE telegram_id = ?")
            .bind(telegram_id)
            .fetch_one(&pool)
            .await?;
        let count: i64 = row.get::<i64, _>("cnt");
        assert_eq!(count, 1);

        Ok(())
    }
}
