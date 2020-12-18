//! Utilities to read server configuration from file.

#[doc(hidden)]
#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application_port: u16,
    pub cache_size: usize,
    pub external_services: ExternalServices,
}

#[doc(hidden)]
#[derive(serde::Deserialize, Clone)]
pub struct ExternalServices {
    pub pokeapi_url: String,
    pub shakespeare_translation_url: String,
}

/// Read the configuration from the given `file_path`.
///
/// The path should not contain the file extension (e.g., just use `configuration`
/// to refer to a file named `configuration.yml`). Supported file types
/// include JSON, YAML, TOML and HJSON.
pub fn get_configuration(file_path: &str) -> Configuration {
    // NOTE: Normally we would prefer to directly use `try_get_configuration`
    // but as far as I could see actix-web does not play nicely with anyhow
    // or eyre, so we are forced to write boilerplate to translate
    // `config::ConfigError` into `std::io::Error`. Since it is not worth
    // the effort, we simply panic instead.
    match try_get_configuration(file_path) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to read configuration: {}", e);
            panic!("Failed to read configuration: {}", e);
        }
    }
}

fn try_get_configuration(file_name: &str) -> Result<Configuration, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration.merge(config::File::with_name(file_name))?;
    configuration.try_into()
}
