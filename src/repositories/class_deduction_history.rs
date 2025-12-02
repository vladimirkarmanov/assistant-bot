use std::fmt;

use sqlx::{Sqlite, Transaction, prelude::FromRow};

#[derive(FromRow)]
pub struct ClassDeductionHistory {
    pub created_at: String,
    pub class_deduction_history_id: i64,
    pub class_id: i64,
    pub user_id: i64,
}

impl fmt::Display for ClassDeductionHistory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.created_at)
    }
}

pub struct ClassDeductionHistoryRepository<'a, 'c> {
    tx: &'a mut Transaction<'c, Sqlite>,
}

impl<'a, 'c> ClassDeductionHistoryRepository<'a, 'c> {
    pub fn new(tx: &'a mut Transaction<'c, Sqlite>) -> Self {
        Self { tx }
    }

    pub async fn create(&mut self, class_id: i64, user_id: i64) -> anyhow::Result<i64> {
        let result = sqlx::query(
            "insert into class_deduction_history (class_id, user_id)
                 values (?, ?)",
        )
        .bind(class_id)
        .bind(user_id)
        .execute(self.tx.as_mut())
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
        .fetch_all(self.tx.as_mut())
        .await?;

        Ok(histories)
    }
}
