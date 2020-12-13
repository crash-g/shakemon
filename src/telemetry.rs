use std::env;

pub fn init_log(env_var_name: &str) {
    if env::var(env_var_name).is_err() {
        env::set_var(env_var_name, "info");
    }
    pretty_env_logger::try_init_timed_custom_env(env_var_name).expect("TODO");
}
