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
) -> Result<Class, Box<dyn std::error::Error + Send + Sync>> {
    let user_id: i64 = sqlx::query_scalar::<_, i64>(
        "select user_id from user where telegram_id = ?",
    )
    .bind(telegram_user_id)
    .fetch_one(db)
    .await?;
    
    let result = sqlx::query(
        "insert into class (name, quantity, user_id)
         values (?, ?, ?)"
    )
    .bind(name)
    .bind(quantity as i64)
    .bind(user_id)
    .execute(db)
    .await?;
    
    let class_id = result.last_insert_rowid();

    let class = sqlx::query_as::<_, Class>(
        "select class_id, name, quantity, user_id from class where class_id = ?",
    )
    .bind(class_id)
    .fetch_one(db)
    .await?;

    Ok(class)
}
