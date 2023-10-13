use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoRest {
    pub url: String,
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub go_rest: GoRest
}

impl Configuration {

    /// Read configuration from file `config.toml`.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to configuration file.
    pub fn read_from_config_file(path: &str) -> Result<Self, ConfigError> {
        // Read configuration from given path.
        let config_builder = Config::builder()
            .add_source(config::File::with_name(path))
            .build()?
        ;

        // Deserialize the result.
        config_builder.try_deserialize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_config_file_failure() {
        assert!(Configuration::read_from_config_file("MADE_UP_PATH").is_err());
    }

    #[test]
    fn test_read_from_config_file_success() {
        let configuration_result = Configuration::read_from_config_file("resources/test/config");
        assert!(configuration_result.is_ok());
        let configuration = configuration_result.unwrap();
        assert_eq!("TEST_URL", configuration.go_rest.url);
        assert_eq!("TEST_TOKEN", configuration.go_rest.token);
    }
}