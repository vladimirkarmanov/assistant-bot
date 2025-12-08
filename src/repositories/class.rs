use std::fmt;

use sqlx::{Sqlite, Transaction, prelude::FromRow};

#[derive(FromRow)]
pub struct Class {
    pub name: String,
    pub class_id: i64,
    pub user_id: i64,
    pub quantity: u8,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.quantity)
    }
}

pub struct ClassRepository<'a, 'c> {
    tx: &'a mut Transaction<'c, Sqlite>,
}

impl<'a, 'c> ClassRepository<'a, 'c> {
    pub fn new(tx: &'a mut Transaction<'c, Sqlite>) -> Self {
        Self { tx }
    }

    pub async fn create(
        &mut self,
        name: String,
        quantity: i64,
        user_id: i64,
    ) -> anyhow::Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "insert into class (name, quantity, user_id)
             values (?, ?, ?)",
        )
        .bind(name)
        .bind(quantity)
        .bind(user_id)
        .execute(self.tx.as_mut())
        .await?;

        let class_id = result.last_insert_rowid();
        Ok(class_id)
    }

    pub async fn update_quantity(&mut self, class_id: i64, quantity: u8) -> anyhow::Result<Class> {
        let updated_class = sqlx::query_as::<_, Class>(
            "update class
            set quantity = ?
            where class_id = ?
            returning class_id, name, quantity, user_id",
        )
        .bind(quantity)
        .bind(class_id)
        .fetch_one(self.tx.as_mut())
        .await?;

        Ok(updated_class)
    }

    pub async fn get_user_class_by_id(
        &mut self,
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
        .fetch_optional(self.tx.as_mut())
        .await?;

        Ok(class)
    }

    pub async fn get_user_classes(&mut self, user_id: i64) -> anyhow::Result<Vec<Class>> {
        let classes: Vec<Class> = sqlx::query_as::<_, Class>(
            "select class_id, name, quantity, user_id
                 from class
                 where user_id = ?",
        )
        .bind(user_id)
        .fetch_all(self.tx.as_mut())
        .await?;
        Ok(classes)
    }
}
