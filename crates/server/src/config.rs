use dotenvy::dotenv;
use std::{env, fmt};

#[derive(PartialEq)]
pub enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}

pub struct Config {
    pub env: AppEnv,
    pub port: u16,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let env = match env::var("APP_ENV") {
            Ok(v) if v.to_lowercase() == "prod" => AppEnv::Prod,
            _ => AppEnv::Dev,
        };

        log::debug!("Running App in {} mode.", env);

        match dotenv() {
            Ok(_) => log::debug!(".env file loaded succesfully."),
            Err(_) => log::warn!(".env file not found."),
        };

        Config {
            env,
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(4000),
            db_url: env::var("DB_URL").expect("DB_URL not set"),
        }
    }
}
