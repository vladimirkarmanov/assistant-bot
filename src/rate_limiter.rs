use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisRateLimiter {
    namespace: String,
    conn: ConnectionManager,
    interval: Duration,
    limit: u16,
}

#[derive(Clone)]
pub struct UserRateLimit {
    pub can_proceed_request: bool,
    pub should_notify_user: bool,
}

impl RedisRateLimiter {
    pub async fn new(
        redis_url: &str,
        limit: u16,
        interval: Duration,
        namespace: &str,
    ) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self {
            conn,
            limit,
            interval,
            namespace: namespace.into(),
        })
    }

    fn build_key(&self, user_id: u64) -> String {
        format!(
            "{}:rate_limit:{}:{}",
            self.namespace,
            user_id,
            self.interval.as_secs()
        )
    }

    pub async fn get_user_current_limit(&self, user_id: u64) -> anyhow::Result<UserRateLimit> {
        let key = self.build_key(user_id);
        let mut conn = self.conn.clone();

        // Increment request count
        let requests_count: u16 = conn.incr(&key, 1).await?;

        // Set TTL
        if requests_count == 1 {
            let _: () = redis::cmd("EXPIRE")
                .arg(&key)
                .arg(i64::try_from(self.interval.as_secs()).unwrap_or(10))
                .query_async(&mut conn)
                .await?;
        }

        Ok(UserRateLimit {
            can_proceed_request: requests_count <= self.limit,
            should_notify_user: requests_count == self.limit + 1,
        })
    }
}
