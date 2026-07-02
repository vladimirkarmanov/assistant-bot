use redis::aio::ConnectionManager;
use std::time::Duration;

/// Atomically increments the per-user counter and sets the expiry on the
/// first request of a window. Doing both in one script guarantees the key
/// can never be left without a TTL (which would rate-limit the user forever).
const RATE_LIMIT_SCRIPT: &str = r"
local current = redis.call('INCR', KEYS[1])
if current == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[1])
end
return current
";

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

        let ttl_secs = i64::try_from(self.interval.as_secs()).unwrap_or(10);
        let requests_count: i64 = redis::Script::new(RATE_LIMIT_SCRIPT)
            .key(&key)
            .arg(ttl_secs)
            .invoke_async(&mut conn)
            .await?;

        let limit = i64::from(self.limit);
        Ok(UserRateLimit {
            can_proceed_request: requests_count <= limit,
            should_notify_user: requests_count == limit + 1,
        })
    }
}
