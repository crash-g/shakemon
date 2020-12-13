use std::env;
use std::net::TcpListener;

const LOG_ENV_VAR: &str = "TEST_LOG";
const CONFIGURATION_FILE: &str = "tests/test_configuration";

#[derive(serde::Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

pub fn spawn_app() -> String {
    init_log();
    let configuration = shakespeare_pokemon::configuration::get_configuration(CONFIGURATION_FILE);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = shakespeare_pokemon::run(listener, configuration.external_services)
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

fn init_log() {
    if env::var(LOG_ENV_VAR).is_err() {
        env::set_var(LOG_ENV_VAR, "warn");
    }
    shakespeare_pokemon::telemetry::init_log(LOG_ENV_VAR);
}
