use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::uow::UnitOfWork;

pub async fn add_user(
    db: Arc<Pool<Sqlite>>,
    telegram_id: i64,
    username: &str,
) -> anyhow::Result<()> {
    let mut uow = UnitOfWork::new(db.as_ref()).await?;
    if !uow.user_repo().exists(telegram_id).await? {
        uow.user_repo().create(telegram_id, username).await?;
        uow.commit().await?;
    }
    Ok(())
}
