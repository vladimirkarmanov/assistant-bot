use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub rate_interval_secs: u64,
    pub rate_limit: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot_token: String,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn from_env() -> Self {
        let builder = config::Config::builder().add_source(
            config::Environment::default()
                .separator("__")
                .try_parsing(true),
        );

        let cfg = builder.build().expect("Failed to build config");
        cfg.try_deserialize().expect("Failed to deserialize config")
    }
}
