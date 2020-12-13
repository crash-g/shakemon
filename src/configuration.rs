#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application_port: u16,
    pub external_services: ExternalServices,
}

#[derive(serde::Deserialize, Clone)]
pub struct ExternalServices {
    pub pokeapi_url: String,
    pub shakespeare_translation_url: String,
}

// NOTE: Normally we would prefer to directly use `try_get_configuration`
// but actix-web does not play nicely with anyhow or eyre, so we are
// forced to write boilerplate to translate `config::ConfigError`
// into `std::io::Error`. Since it is not worth the effort,
// we simply panic instead.
pub fn get_configuration(file_name: &str) -> Configuration {
    match try_get_configuration(file_name) {
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
