use serde::Deserialize;

const DEFAULT_LISTENER_HOST: &str = "127.0.0.1";
const DEFAULT_LISTENER_PORT: u16 = 8080;

#[derive(Deserialize)]
pub struct ListenerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub listener: ListenerConfig
}

impl Config {
    pub fn default() -> Self {
        Self {
            listener: ListenerConfig {
                host: DEFAULT_LISTENER_HOST.to_string(),
                port: DEFAULT_LISTENER_PORT,
            }
        }
    }

    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        config.try_deserialize()
    }
}