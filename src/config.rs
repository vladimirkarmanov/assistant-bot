use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot_token: String,
    pub database: DatabaseConfig,
    pub debug: bool,
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
