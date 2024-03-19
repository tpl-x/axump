use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub log_dir: String,
    pub log_name_prefix: String,
    pub log_level: String,
    pub host: String,
    pub port: u16,

}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            log_dir: "logs".to_string(),
            log_name_prefix: "log".to_string(),
            log_level: "info".to_string(),
            host: "0.0.0.0".into(),
            port: 3000,
        }
    }
}

pub async fn parse_config(config_path: Option<&str>) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let conf = config_path.unwrap_or("config.yaml");
    let mut file = File::open(conf).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: AppConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}