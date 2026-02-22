use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_port: String,
    pub groq_api_key: String,
    pub google_project_id: String,
    pub api_key_value: String,
    pub vercel_domain: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        s.try_deserialize()
    }
}
