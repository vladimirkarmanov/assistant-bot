use std::{fmt, ops::DerefMut};

use chrono::{Datelike, NaiveDateTime};
use sqlx::{SqliteConnection, prelude::FromRow};

use crate::utils::get_russian_weekday_name;

#[derive(FromRow)]
pub struct ClassDeductionHistory {
    pub created_at: String,
    pub class_deduction_history_id: i64,
    pub class_id: i64,
    pub user_id: i64,
}

impl fmt::Display for ClassDeductionHistory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dt = NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted_date = dt.format("%d.%m.%Y %H:%M").to_string();
        write!(
            f,
            "{} ({})",
            formatted_date,
            get_russian_weekday_name(dt.weekday(), true)
        )
    }
}

pub struct ClassDeductionHistoryRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ClassDeductionHistoryRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub async fn create(&mut self, class_id: i64, user_id: i64) -> anyhow::Result<i64> {
        let result = sqlx::query(
            "insert into class_deduction_history (class_id, user_id)
                 values (?, ?)",
        )
        .bind(class_id)
        .bind(user_id)
        .execute(self.conn.deref_mut())
        .await?;

        let id = result.last_insert_rowid();
        Ok(id)
    }

    pub async fn get_histories(
        &mut self,
        class_id: i64,
        user_id: i64,
    ) -> anyhow::Result<Vec<ClassDeductionHistory>> {
        let histories: Vec<ClassDeductionHistory> = sqlx::query_as::<_, ClassDeductionHistory>(
            "select class_deduction_history_id, class_id, user_id, created_at
             from class_deduction_history
             where user_id = ?
             and class_id = ?",
        )
        .bind(user_id)
        .bind(class_id)
        .fetch_all(self.conn.deref_mut())
        .await?;

        Ok(histories)
    }
}
