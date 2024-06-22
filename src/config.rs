use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub html_dir: String,
    pub network: NetworkConfig,
    pub throttling: ThrottlingConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    pub max_download_speed: String,
    pub max_upload_speed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThrottlingConfig {
    pub enabled: bool,
    pub burst_limit: String,
    pub burst_duration: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if !std::path::Path::new("config.toml").exists() {
            Self::create_default_config()?;
        }

        let config_data = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_data)?;
        Ok(config)
    }

    fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
        let default_config = Config {
            network: NetworkConfig {
                max_download_speed: "1Mbps".into(),
                max_upload_speed: "512kbps".into(),
            },
            throttling: ThrottlingConfig {
                enabled: true,
                burst_limit: "200kbps".into(),
                burst_duration: "5s".into(),
            },
            html_dir: "./html".into(),
        };

        let config_string = toml::to_string(&default_config)?;
        let mut file = fs::File::create("config.toml")?;
        file.write_all(config_string.as_bytes())?;
        Ok(())
    }

    pub fn handle_config_command(args: &[String]) {
        match args.get(0).map(|s| s.as_str()) {
            Some("show") => {
                let config = Config::load().expect("Failed to load configuration");
                println!("{:#?}", config);
            }
            _ => eprintln!("Unknown config command"),
        }
    }
}
