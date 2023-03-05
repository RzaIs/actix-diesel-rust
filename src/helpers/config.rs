use config::{Config, Environment};
use serde::Deserialize;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct Secrets {
    pub database_url: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_secret: String,
    pub argon_salt: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub pool_size: u32,
    pub rust_log: String
}

impl Secrets {
    pub fn from_env() -> Self {
        let env = Environment::default();
        Config::builder()
            .add_source(env)
            .build()
            .expect("Error in building Config from env")
            .try_deserialize()
            .expect("Error in deserializing Secrets from Config")
    }
}

impl Settings {
    pub fn from_env() -> Self {
        let env = Environment::with_prefix("setting")
            .prefix_separator("_");

        Config::builder()
            .add_source(env)
            .build()
            .expect("Error in building Config from env")
            .try_deserialize()
            .expect("Error in deserializing Settings from Config")
    }
}