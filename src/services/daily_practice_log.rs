use std::sync::Arc;

use anyhow::bail;
use sqlx::{Pool, Sqlite};

use crate::{errors::*, repositories::daily_practice_log::DailyPracticeLog, uow::UnitOfWork};

pub async fn add_daily_practice_entry(
    db_pool: Arc<Pool<Sqlite>>,
    minutes: u16,
    telegram_user_id: i64,
) -> anyhow::Result<i64> {
    let mut uow = UnitOfWork::new_transactional(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .await?
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let daily_practice_entry_id = match uow
        .daily_practice_log_repo()
        .await?
        .create(minutes, user_id)
        .await
    {
        Ok(daily_practice_entry_id) => daily_practice_entry_id,
        Err(_) => bail!(SomethingWentWrongError),
    };

    uow.commit().await?;
    Ok(daily_practice_entry_id)
}

pub async fn get_daily_practice_log_history(
    db_pool: Arc<Pool<Sqlite>>,
    telegram_user_id: i64,
) -> anyhow::Result<Vec<DailyPracticeLog>> {
    let mut uow = UnitOfWork::new_readonly(db_pool.as_ref());
    let user_id = match uow
        .user_repo()
        .await?
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let records = uow
        .daily_practice_log_repo()
        .await?
        .get_all(user_id)
        .await?;
    Ok(records)
}
