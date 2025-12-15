use crate::repositories::{
    class::ClassRepository, class_deduction_history::ClassDeductionHistoryRepository,
    daily_practice_log::DailyPracticeLogRepository, user::UserRepository,
};
use sqlx::{Pool, Sqlite, SqliteConnection, SqlitePool, Transaction, pool::PoolConnection};

enum UowContext<'a> {
    ReadOnly {
        pool: &'a Pool<Sqlite>,
        conn: Option<PoolConnection<Sqlite>>,
    },
    Transactional(Option<Transaction<'a, Sqlite>>),
}

pub struct UnitOfWork<'a> {
    context: UowContext<'a>,
}

impl<'a> UnitOfWork<'a> {
    pub fn new_readonly(pool: &'a SqlitePool) -> Self {
        Self {
            context: UowContext::ReadOnly { pool, conn: None },
        }
    }

    pub async fn new_transactional(pool: &'a SqlitePool) -> Result<Self, sqlx::Error> {
        let tx = pool.begin().await?;
        Ok(Self {
            context: UowContext::Transactional(Some(tx)),
        })
    }

    async fn connection(&mut self) -> Result<&mut SqliteConnection, sqlx::Error> {
        match &mut self.context {
            UowContext::Transactional(tx_opt) => {
                let tx = tx_opt.as_mut().expect("Transaction consumed");
                Ok(tx)
            }
            UowContext::ReadOnly { pool, conn } => {
                if conn.is_none() {
                    let c = pool.acquire().await?;
                    *conn = Some(c);
                }
                Ok(conn.as_mut().unwrap())
            }
        }
    }

    pub async fn commit(mut self) -> anyhow::Result<(), sqlx::Error> {
        if let UowContext::Transactional(tx_opt) = &mut self.context {
            if let Some(tx) = tx_opt.take() {
                tx.commit().await?;
            }
        }
        Ok(())
    }

    pub async fn user_repo(&mut self) -> Result<UserRepository<'_>, sqlx::Error> {
        let conn = self.connection().await?;
        Ok(UserRepository::new(conn))
    }

    pub async fn class_repo(&mut self) -> Result<ClassRepository<'_>, sqlx::Error> {
        let conn = self.connection().await?;
        Ok(ClassRepository::new(conn))
    }

    pub async fn class_deduction_history_repo(
        &mut self,
    ) -> Result<ClassDeductionHistoryRepository<'_>, sqlx::Error> {
        let conn = self.connection().await?;
        Ok(ClassDeductionHistoryRepository::new(conn))
    }

    pub async fn daily_practice_log_repo(
        &mut self,
    ) -> Result<DailyPracticeLogRepository<'_>, sqlx::Error> {
        let conn = self.connection().await?;
        Ok(DailyPracticeLogRepository::new(conn))
    }
}
