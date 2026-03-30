use serde::Deserialize;
use config::{Config as ConfigRs, ConfigError, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // 尝试多个配置文件位置
        let config_paths = [
            "config/finance",           // 从项目根目录运行
            "../config/finance",        // 从子目录运行
            "../../config/finance",     // 从更深的子目录运行
        ];

        let mut builder = ConfigRs::builder();
        
        let mut found = false;
        for path in &config_paths {
            if std::path::Path::new(&format!("{}.json", path)).exists() ||
               std::path::Path::new(&format!("{}.toml", path)).exists() ||
               std::path::Path::new(&format!("{}.yaml", path)).exists() {
                builder = builder.add_source(File::with_name(path));
                found = true;
                break;
            }
        }

        if !found {
            // 如果找不到配置文件，使用环境变量或默认值
            builder = builder
                .add_source(File::with_name("config/finance").required(false))
                .add_source(File::with_name("../config/finance").required(false));
        }

        let config = builder
            .add_source(config::Environment::with_prefix("FINANCE"))
            .set_default("database_url", "postgres://postgres:postgres@localhost:5432/finance")?
            .set_default("server_host", "0.0.0.0")?
            .set_default("server_port", 8082)?
            .build()?;
        
        config.try_deserialize()
    }
}
