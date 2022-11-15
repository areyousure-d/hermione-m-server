use config::ConfigError;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i16,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i16,
    pub dbname: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub pg: DbConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
