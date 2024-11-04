use config::{Config, ConfigError, ConfigBuilder, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub influx_url: String,
    pub influx_token: String,
    pub influx_org: String,
    pub influx_bucket: String,
    pub refresh_rate: u64,
    pub identifier: String,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name("config"))
        .add_source(Environment::with_prefix("MONITOR").separator("_"));

    builder.build()?.try_deserialize()
}
