use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Euroleague {
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub euroleague: Euroleague,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("Settings"));

        builder.build()?.try_deserialize()
    }
}
