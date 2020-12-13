#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application_port: u16,
}

// NOTE: Normally we would prefer to directly use `try_get_configuration`
// but actix-web does not play nicely with anyhow or eyre, so we are
// forced to write boilerplate to translate `config::ConfigError`
// into `std::io::Error`. Since it is not worth the effort,
// we simply panic instead.
pub fn get_configuration() -> Configuration {
    match try_get_configuration() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to read configuration: {}", e);
            panic!("Failed to read configuration: {}", e);
        }
    }
}

fn try_get_configuration() -> Result<Configuration, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration.merge(config::File::with_name("configuration"))?;
    configuration.try_into()
}
