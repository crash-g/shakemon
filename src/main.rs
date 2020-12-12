use std::net::TcpListener;
use shakespeare_pokemon::run;
use shakespeare_pokemon::configuration::get_configuration;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
