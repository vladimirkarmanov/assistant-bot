use anyhow::bail;
use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct Class {
    pub class_id: i64,
    pub name: String,
    pub quantity: u8,
    pub user_id: i64,
}

pub async fn add_class(
    db: &Pool<Sqlite>,
    name: String,
    quantity: u8,
    telegram_user_id: i64,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let user_id: i64 =
        sqlx::query_scalar::<_, i64>("select user_id from user where telegram_id = ?")
            .bind(telegram_user_id)
            .fetch_one(db)
            .await?;

    let result = sqlx::query(
        "insert into class (name, quantity, user_id)
         values (?, ?, ?)",
    )
    .bind(name)
    .bind(quantity as i64)
    .bind(user_id)
    .execute(db)
    .await?;

    let class_id = result.last_insert_rowid();
    Ok(class_id)
}

pub async fn get_classes_by_user_id(
    db: &Pool<Sqlite>,
    telegram_user_id: i64,
) -> Result<Vec<Class>, Box<dyn std::error::Error + Send + Sync>> {
    let user_id: i64 =
        sqlx::query_scalar::<_, i64>("select user_id from user where telegram_id = ?")
            .bind(telegram_user_id)
            .fetch_one(db)
            .await?;

    let classes: Vec<Class> = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id
         from class
         where user_id = ?",
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(classes)
}

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Не удалось списать занятие. Количество доступных занятий {0}")]
struct NotEnoughClassQuantityToChargeError(u8);

pub async fn charge_class(
    db: &Pool<Sqlite>,
    class_id: i64,
    telegram_user_id: i64,
) -> anyhow::Result<Class> {
    let user_id: i64 =
        sqlx::query_scalar::<_, i64>("select user_id from user where telegram_id = ?")
            .bind(telegram_user_id)
            .fetch_one(db)
            .await?;

    let class: Class = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id
         from class
         where class_id = ? and user_id = ?",
    )
    .bind(class_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

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
    .fetch_one(db)
    .await?;

    Ok(updated_class)
}


pub async fn update_class_quantity(
    db: &Pool<Sqlite>,
    class_id: i64,
    telegram_user_id: i64,
    quantity: u8
) -> anyhow::Result<Class> {
    let user_id: i64 =
        sqlx::query_scalar::<_, i64>("select user_id from user where telegram_id = ?")
            .bind(telegram_user_id)
            .fetch_one(db)
            .await?;

    let class: Class = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id
         from class
         where class_id = ? and user_id = ?",
    )
    .bind(class_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    let updated_class = sqlx::query_as::<_, Class>(
        "update class
        set quantity = ?
        where class_id = ?
        returning class_id, name, quantity, user_id",
    )
    .bind(quantity)
    .bind(class.class_id)
    .fetch_one(db)
    .await?;

    Ok(updated_class)
}
