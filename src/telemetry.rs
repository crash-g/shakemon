//! Utilities for tuning telemetry in the server.

use std::env;

/// Initialize the logger from the value of the `env_var_name` environment variable.
///
/// Please see
/// [the documentation of env_logger](https://docs.rs/env_logger/0.7.1/env_logger/index.html)
/// for a description of the values that it accepts.
///
/// If the variable is not set, the default level is `info` restricted to
/// this crate's logs only.
pub fn init_log(env_var_name: &str) {
    if env::var(env_var_name).is_err() {
        env::set_var(env_var_name, "shakemon=info");
    }
    if pretty_env_logger::try_init_timed_custom_env(env_var_name).is_err() {
        log::warn!("The log has already been initialized!")
    }
}
