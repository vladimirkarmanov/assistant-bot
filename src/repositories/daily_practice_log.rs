use std::{fmt, ops::DerefMut};

use chrono::{Datelike, NaiveDateTime};
use sqlx::{SqliteConnection, prelude::FromRow};

use crate::utils::get_russian_weekday_name;

#[derive(FromRow)]
pub struct DailyPracticeLog {
    pub created_at: String,
    pub user_id: i64,
    pub minutes: u16,
}

impl fmt::Display for DailyPracticeLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dt = NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted_date = dt.format("%d.%m.%Y").to_string();
        write!(
            f,
            "{} ({}) - {} мин",
            formatted_date,
            get_russian_weekday_name(dt.weekday(), true),
            self.minutes
        )
    }
}
pub struct DailyPracticeLogRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> DailyPracticeLogRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub async fn create(&mut self, minutes: u16, user_id: i64) -> anyhow::Result<i64> {
        let result = sqlx::query(
            "insert into daily_practice_log (minutes, user_id)
             values (?, ?)",
        )
        .bind(minutes)
        .bind(user_id)
        .execute(self.conn.deref_mut())
        .await?;

        let class_id = result.last_insert_rowid();
        Ok(class_id)
    }

    pub async fn get_all(&mut self, user_id: i64) -> anyhow::Result<Vec<DailyPracticeLog>> {
        let records: Vec<DailyPracticeLog> = sqlx::query_as::<_, DailyPracticeLog>(
            "select daily_practice_log_id, minutes, user_id, created_at
             from daily_practice_log
             where user_id = ?",
        )
        .bind(user_id)
        .fetch_all(self.conn.deref_mut())
        .await?;

        Ok(records)
    }
}
