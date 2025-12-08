use std::sync::Arc;

use anyhow::bail;
use sqlx::{Pool, Sqlite};

use crate::{
    errors::*,
    repositories::{class::Class, class_deduction_history::ClassDeductionHistory},
    uow::UnitOfWork,
};

pub async fn add_class(
    db_pool: Arc<Pool<Sqlite>>,
    name: String,
    quantity: u8,
    telegram_user_id: i64,
) -> anyhow::Result<i64> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let class_id = match uow
        .class_repo()
        .create(name, quantity as i64, user_id)
        .await
    {
        Ok(class_id) => class_id,
        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                match db_err.code().as_deref() {
                    Some("2067") | Some("1555") => {
                        bail!(DuplicateClassNameError);
                    }
                    _ => {
                        bail!(SomethingWentWrongError);
                    }
                }
            }
            bail!(SomethingWentWrongError);
        }
    };

    uow.commit().await?;
    Ok(class_id)
}

pub async fn get_classes_by_user_id(
    db_pool: Arc<Pool<Sqlite>>,
    telegram_user_id: i64,
) -> anyhow::Result<Vec<Class>> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let classes = uow.class_repo().get_user_classes(user_id).await?;
    Ok(classes)
}

pub async fn get_class_deduction_histories(
    db_pool: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
) -> anyhow::Result<Vec<ClassDeductionHistory>> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let histories = uow
        .class_deduction_history_repo()
        .get_histories(class_id, user_id)
        .await?;
    Ok(histories)
}

pub async fn deduct_class(
    db_pool: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
) -> anyhow::Result<Class> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let class = match uow
        .class_repo()
        .get_user_class_by_id(class_id, user_id)
        .await?
    {
        Some(c) => c,
        None => {
            bail!(ClassNotFoundError);
        }
    };

    if class.quantity == 0 {
        bail!(NotEnoughClassQuantityToDeductError(class.quantity));
    }

    let new_quantity = class.quantity - 1;
    let updated_class = uow
        .class_repo()
        .update_quantity(class.class_id, new_quantity)
        .await?;

    uow.commit().await?;
    Ok(updated_class)
}

pub async fn add_class_deduction_history(
    db_pool: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
) -> anyhow::Result<()> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    uow.class_deduction_history_repo()
        .create(class_id, user_id)
        .await?;

    uow.commit().await?;
    Ok(())
}

pub async fn update_class_quantity(
    db_pool: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
    quantity: u8,
) -> anyhow::Result<Class> {
    let mut uow = UnitOfWork::new(db_pool.as_ref()).await?;
    let user_id = match uow
        .user_repo()
        .get_user_by_telegram_id(telegram_user_id)
        .await?
    {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let class = match uow
        .class_repo()
        .get_user_class_by_id(class_id, user_id)
        .await?
    {
        Some(c) => c,
        None => {
            bail!(ClassNotFoundError);
        }
    };

    let updated_class = uow
        .class_repo()
        .update_quantity(class.class_id, quantity)
        .await?;

    uow.commit().await?;
    Ok(updated_class)
}
