use shakespeare_pokemon::configuration::get_configuration;
use shakespeare_pokemon::run;
use shakespeare_pokemon::telemetry::init_log;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_log("RUST_LOG");
    let configuration = get_configuration();
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
