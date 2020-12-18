use shakespeare_pokemon::configuration::ExternalServices;
use std::env;
use std::net::TcpListener;
use wiremock::MockServer;

const LOCALHOST: &str = "127.0.0.1";
const LOG_ENV_VAR: &str = "TEST_LOG";
const LOG_LEVEL: &str = "fatal";

#[allow(dead_code)]
pub fn spawn_app() -> String {
    init_log();
    let external_services = ExternalServices {
        pokeapi_url: "N/A".to_string(),
        shakespeare_translation_url: "N/A".to_string(),
    };
    start_background_server(external_services)
}

#[allow(dead_code)]
pub async fn spawn_app_with_mocked_external_services() -> (String, MockServer, MockServer) {
    init_log();
    let mock_pokeapi_server = MockServer::start().await;
    let mock_shakespeare_server = MockServer::start().await;
    let external_services = ExternalServices {
        pokeapi_url: mock_pokeapi_server.uri(),
        shakespeare_translation_url: mock_shakespeare_server.uri(),
    };
    (
        start_background_server(external_services),
        mock_pokeapi_server,
        mock_shakespeare_server,
    )
}

fn start_background_server(external_services: ExternalServices) -> String {
    let cache_size = 10;
    let listener =
        TcpListener::bind(&format!("{}:0", LOCALHOST)).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = shakespeare_pokemon::run(listener, cache_size, external_services)
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", LOCALHOST, port)
}

fn init_log() {
    if env::var(LOG_ENV_VAR).is_err() {
        env::set_var(LOG_ENV_VAR, LOG_LEVEL);
    }
    shakespeare_pokemon::telemetry::init_log(LOG_ENV_VAR);
}
