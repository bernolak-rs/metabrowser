use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub brave_api_key: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            brave_api_key: env::var("BRAVE_API_KEY")
                .map_err(|_| anyhow::anyhow!("BRAVE_API_KEY not set"))?,
        })
    }
}
