use crate::repositories::{
    class::ClassRepository, class_deduction_history::ClassDeductionHistoryRepository,
    daily_practice_log::DailyPracticeLogRepository, user::UserRepository,
};
use sqlx::{Pool, Sqlite, Transaction};

pub struct UnitOfWork<'c> {
    tx: Transaction<'c, Sqlite>,
}

impl<'c> UnitOfWork<'c> {
    pub async fn new(pool: &'c Pool<Sqlite>) -> anyhow::Result<Self, sqlx::Error> {
        let tx = pool.begin().await?;
        Ok(Self { tx })
    }

    pub async fn commit(self) -> anyhow::Result<(), sqlx::Error> {
        self.tx.commit().await
    }

    pub fn user_repo(&mut self) -> UserRepository<'_, 'c> {
        UserRepository::new(&mut self.tx)
    }

    pub fn class_repo(&mut self) -> ClassRepository<'_, 'c> {
        ClassRepository::new(&mut self.tx)
    }

    pub fn class_deduction_history_repo(
        &mut self,
    ) -> crate::repositories::class_deduction_history::ClassDeductionHistoryRepository<'_, 'c> {
        ClassDeductionHistoryRepository::new(&mut self.tx)
    }

    pub fn daily_practice_log_repo(&mut self) -> DailyPracticeLogRepository<'_, 'c> {
        DailyPracticeLogRepository::new(&mut self.tx)
    }
}
