#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration.merge(config::File::with_name("configuration"))?;
    configuration.try_into()
}
