use std::sync::Arc;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub enum Environment {
    Local,
    Production
}
#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub environment: Environment,
    pub database_url: String,
    pub app_port: u16,
    pub max_db_connection: u8,
}

impl Environment {
    pub fn environment_as_string(&self) -> &'static str {
        match self {
            Environment::Local => "Local",
            Environment::Production => "Production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Local" => Ok(Environment::Local),
            "Production" => Ok(Environment::Production),
            _ => Err("Invalid environment"),
        }
    }
}

impl Configuration {

}
pub type AppConfig = Arc<Configuration>;
pub fn load_env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{name}: {e}"))
        .expect("Missing environment variable")
}