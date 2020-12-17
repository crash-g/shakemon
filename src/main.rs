use shakespeare_pokemon::configuration::get_configuration;
use shakespeare_pokemon::run;
use shakespeare_pokemon::telemetry::init_log;
use std::net::TcpListener;

const LOG_ENV_VAR: &str = "RUST_LOG";
const CONFIGURATION_FILE: &str = "configuration";
const HOST: &str = "127.0.0.1";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_log(LOG_ENV_VAR);
    let configuration = get_configuration(CONFIGURATION_FILE);
    let address = format!("{}:{}", HOST, configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(
        listener,
        configuration.cache_size,
        configuration.external_services,
    )?
    .await
}
