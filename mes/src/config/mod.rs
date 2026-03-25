use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> Self {
        config::Config::builder()
            .set_default("database.url", "postgres://postgres:123123@localhost:5432/mes")
            .expect("Failed to set default database url")
            .set_default("server.host", "0.0.0.0")
            .expect("Failed to set default server host")
            .set_default("server.port", 8082)
            .expect("Failed to set default server port")
            .add_source(config::Environment::with_prefix("MES"))
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize config")
    }
}
