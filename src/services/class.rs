use std::{fmt, sync::Arc};

use anyhow::bail;
use sqlx::{Pool, Sqlite, prelude::FromRow};

use crate::{errors::*, services::user::get_user_by_telegram_id};

#[derive(FromRow)]
pub struct Class {
    pub class_id: i64,
    pub name: String,
    pub quantity: u8,
    pub user_id: i64,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.quantity)
    }
}

pub async fn add_class(
    db: Arc<Pool<Sqlite>>,
    name: String,
    quantity: u8,
    telegram_user_id: i64,
) -> anyhow::Result<i64> {
    let user_id = match get_user_by_telegram_id(db.clone(), telegram_user_id).await? {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let result = sqlx::query(
        "insert into class (name, quantity, user_id)
         values (?, ?, ?)",
    )
    .bind(name)
    .bind(quantity as i64)
    .bind(user_id)
    .execute(db.as_ref())
    .await?;

    let class_id = result.last_insert_rowid();
    Ok(class_id)
}

pub async fn get_classes_by_user_id(
    db: Arc<Pool<Sqlite>>,
    telegram_user_id: i64,
) -> anyhow::Result<Vec<Class>> {
    let user_id = match get_user_by_telegram_id(db.clone(), telegram_user_id).await? {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let classes: Vec<Class> = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id
         from class
         where user_id = ?",
    )
    .bind(user_id)
    .fetch_all(db.as_ref())
    .await?;

    Ok(classes)
}

pub async fn get_class_by_id(
    db: Arc<Pool<Sqlite>>,
    class_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<Class>> {
    let class: Option<Class> = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id
             from class
             where class_id = ? and user_id = ?",
    )
    .bind(class_id)
    .bind(user_id)
    .fetch_optional(db.as_ref())
    .await?;

    Ok(class)
}

pub async fn charge_class(
    db: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
) -> anyhow::Result<Class> {
    let user_id = match get_user_by_telegram_id(db.clone(), telegram_user_id).await? {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let class = match get_class_by_id(db.clone(), class_id, user_id).await? {
        Some(c) => c,
        None => {
            bail!(ClassNotFoundError);
        }
    };

    if class.quantity == 0 {
        bail!(NotEnoughClassQuantityToChargeError(class.quantity));
    }

    let new_quantity = class.quantity - 1;
    let updated_class = sqlx::query_as::<_, Class>(
        "update class
        set quantity = ?
        where class_id = ?
        returning class_id, name, quantity, user_id",
    )
    .bind(new_quantity)
    .bind(class.class_id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(updated_class)
}

pub async fn update_class_quantity(
    db: Arc<Pool<Sqlite>>,
    class_id: i64,
    telegram_user_id: i64,
    quantity: u8,
) -> anyhow::Result<Class> {
    let user_id = match get_user_by_telegram_id(db.clone(), telegram_user_id).await? {
        Some(u) => u.user_id,
        None => {
            bail!(UserNotFoundError);
        }
    };

    let class = match get_class_by_id(db.clone(), class_id, user_id).await? {
        Some(c) => c,
        None => {
            bail!(ClassNotFoundError);
        }
    };

    let updated_class = sqlx::query_as::<_, Class>(
        "update class
        set quantity = ?
        where class_id = ?
        returning class_id, name, quantity, user_id",
    )
    .bind(quantity)
    .bind(class.class_id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(updated_class)
}
