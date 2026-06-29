use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub api_host: String,
    pub api_port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let api_host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let api_port = env::var("API_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        Ok(Self {
            database_url,
            api_host,
            api_port,
        })
    }
}
