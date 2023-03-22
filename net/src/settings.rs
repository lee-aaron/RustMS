use config::{Config, ConfigError, File, FileFormat};

#[derive(Debug, Deserialize)]
pub struct Login {
    pub pin_required: bool,
    pub gender_required: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub login: Login,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::new("config/login_server_config", FileFormat::Toml))
            .build()
            .unwrap();
        s.try_deserialize()
    }
}
