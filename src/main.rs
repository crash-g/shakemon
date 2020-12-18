//! A server that offers Pokemon descriptions written
//! using Shakespeare's style.
//!
//! ## How to run
//!
//! If using cargo, use `cargo run --release`. If using the executable
//! file directly, just execute it (see below for customization options).
//!
//! ## Customization
//!
//! ### Compile-time customization
//!
//! Three variables are available for compile-time customization:
//! - `LOG_ENV_VAR` → The name of the environment variable which controls the log level
//! - `CONFIGURATION_FILE` → The path to the configuration file which is parsed on startup
//! - `HOST` → The host that the server will listen to
//!
//! ### Environment variable customization
//!
//! The `RUST_LOG` variable is the environment variable that by default controls
//! the log level. Please see
//! [the documentation of env_logger](https://docs.rs/env_logger/0.7.1/env_logger/index.html)
//! for a description of the values that it accepts.
//!
//! ### Configuration file
//!
//! The application by default expects a `configuration.yml` file in the same
//! folder from where it is launched. Please see the bundled configuration file for
//! a description of the available configuration properties.

use shakemon::configuration::get_configuration;
use shakemon::run;
use shakemon::telemetry::init_log;
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
    log::info!(
        "Starting service on {}:{}",
        HOST,
        configuration.application_port
    );
    run(
        listener,
        configuration.cache_size,
        configuration.external_services,
    )?
    .await
}
