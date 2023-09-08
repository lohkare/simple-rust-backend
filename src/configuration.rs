use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoRest {
    pub base_url: String,
    pub bearer_token: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub go_rest: GoRest
}

impl Configuration {

    /// Read configuration from file `Config.toml`.
    pub fn read_from_config_file() -> Result<Self, ConfigError> {
        let config_builder = Config::builder()
            .add_source(File::with_name("Config"))
            .build()?
        ;

        config_builder.try_deserialize()
    }
}