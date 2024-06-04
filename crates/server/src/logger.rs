pub fn init_logger() {
    let env = env_logger::Env::default()
        .filter_or("APP_LOG_LEVEL", "trace")
        .write_style_or("APP_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}
