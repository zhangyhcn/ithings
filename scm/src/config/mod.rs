use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn load() -> Self {
        config::Config::builder()
            .set_default("server.host", "0.0.0.0")
            .unwrap()
            .set_default("server.port", 8081)
            .unwrap()
            .set_default("database.url", "postgres://postgres:123123@localhost:5432/scm")
            .unwrap()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("SCM"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
