use config::ConfigError;
use std::ops::Deref;

#[derive(Debug, Deserialize, Default)]
pub struct JWTSettings {
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub expiration: Option<i64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct DatabaseSettings {
    #[serde(default = "default_database_url")]
    pub url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ServiceSettings {
    #[serde(default)]
    pub logfile: Option<String>,
    #[serde(default = "default_pidfile")]
    pub pidfile: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub database: DatabaseSettings,
    #[serde(default)]
    pub service: ServiceSettings,
    #[serde(default)]
    pub jwt: JWTSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = config::Config::default();
        config.merge(config::File::from_str(
            include_str!("default-settings.toml"),
            config::FileFormat::Toml,
        ))?;
        config.try_into()
    }

    pub fn with_file<S: Deref<Target = str>>(config_file: &Option<S>) -> Result<Self, ConfigError> {
        let mut config = config::Config::default();
        config.merge(config::File::from_str(
            include_str!("default-settings.toml"),
            config::FileFormat::Toml,
        ))?;
        if let Some(config_file) = config_file {
            config.merge(config::File::new(&config_file, config::FileFormat::Toml))?;
        }
        config.try_into()
    }
}

fn default_database_url() -> String {
    return "roompla.sqlite".to_string();
}

fn default_pidfile() -> String {
    return "/tmp/roompla.pid".to_string();
}
