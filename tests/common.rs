use std::net::TcpListener;

const LOG_ENV_VAR: &str = "TEST_LOG";

pub fn spawn_app() -> String {
    shakespeare_pokemon::telemetry::init_log(LOG_ENV_VAR);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = shakespeare_pokemon::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[derive(serde::Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}
