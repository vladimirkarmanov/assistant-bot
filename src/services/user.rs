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
