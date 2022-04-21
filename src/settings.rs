use serde::Deserialize;
use config::{Config, Environment};
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app_port: Option<u16>,
    pub database_url: String,
    pub jwt_secret: String,
    pub rust_log: Option<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self, Error> {
        let cfg = Config::builder()
            .add_source(Environment::default().try_parsing(true))
            .build()?;

        cfg.try_deserialize().map_err(|e| Error::ConfigFailed(e))
        // cfg.try_deserialize()?
    }
}

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn test_load_config() {
        dotenv::dotenv().ok();
        let cfg = Settings::from_env().unwrap();
        println!("{:#?}", cfg);
    }
}