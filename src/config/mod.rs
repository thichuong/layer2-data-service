use std::env;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub redis_url: String,
    pub taapi_secret: String,
    pub cmc_api_key: Option<String>,
    pub finnhub_api_key: Option<String>,
    pub debug: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        let taapi_secret = env::var("TAAPI_SECRET")
            .context("TAAPI_SECRET is required but not set")?;

        let cmc_api_key = env::var("CMC_API_KEY").ok();
        let finnhub_api_key = env::var("FINNHUB_API_KEY").ok();

        let debug = env::var("DEBUG")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        Ok(Self {
            host,
            redis_url,
            taapi_secret,
            cmc_api_key,
            finnhub_api_key,
            debug,
        })
    }
}
