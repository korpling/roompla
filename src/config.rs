use config::ConfigError;
use std::ops::Deref;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct JWTSettings {
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub expiration: Option<i64>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct DatabaseSettings {
    #[serde(default = "default_database_url")]
    pub url: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServiceSettings {
    #[serde(default = "default_service_port")]
    pub port: u16,

    #[serde(default)]
    pub logfile: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct LogSettings {
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct LDAPSettings {
    #[serde(default = "default_ldap_url")]
    pub url: String,
    #[serde(default = "default_ldap_organization")]
    pub organization: String,
    #[serde(default = "default_ldap_filter")]
    pub filter: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Settings {
    #[serde(default)]
    pub service: ServiceSettings,
    #[serde(default)]
    pub database: DatabaseSettings,
    #[serde(default)]
    pub ldap: LDAPSettings,
    #[serde(default)]
    pub jwt: JWTSettings,
    #[serde(default)]
    pub log: LogSettings,
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

    pub fn with_file<S: Deref<Target = str>>(config_file: S) -> Result<Self, ConfigError> {
        let mut config = config::Config::default();
        config.merge(config::File::from_str(
            include_str!("default-settings.toml"),
            config::FileFormat::Toml,
        ))?;
        config.merge(config::File::new(&config_file, config::FileFormat::Toml))?;
        config.try_into()
    }
}

fn default_database_url() -> String {
    return "roompla.sqlite".to_string();
}

fn default_ldap_url() -> String {
    return "ldaps://ldapmaster.cms.hu-berlin.de".to_string();
}

fn default_ldap_organization() -> String {
    return "ou=users,ou=Benutzerverwaltung,ou=Computer- und Medienservice,o=Humboldt-Universitaet zu Berlin,c=DE".to_string();
}

fn default_ldap_filter() -> String {
    return "(uid=*)".to_string();
}

fn default_service_port() -> u16 {
    return 5050;
}
