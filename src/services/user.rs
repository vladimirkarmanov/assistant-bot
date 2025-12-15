use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::uow::UnitOfWork;

pub async fn add_user(
    db_pool: Arc<Pool<Sqlite>>,
    telegram_id: i64,
    username: &str,
) -> anyhow::Result<()> {
    let mut uow = UnitOfWork::new_transactional(db_pool.as_ref()).await?;
    if !uow.user_repo().await?.exists(telegram_id).await? {
        uow.user_repo().await?.create(telegram_id, username).await?;
        uow.commit().await?;
    }
    Ok(())
}
