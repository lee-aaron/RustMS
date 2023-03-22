use config::{Config, ConfigError, File, FileFormat};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder().add_source(File::new("config/db_config", FileFormat::Toml)).build().unwrap();
        s.try_deserialize()
    }
}
