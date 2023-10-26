use std::fs;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME : &str = "http-server.json";

#[derive(Serialize, Deserialize )]
pub struct Config {
    server_ip: String,
    port: u16
}

impl Config {
    pub fn server_ip(&self) -> String {
        self.server_ip.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn load_config() -> Config {
    let config_raw = match fs::read_to_string(CONFIG_FILE_NAME) {
        Ok(c) => c,
        Err(_) => {
            println!("Please configure HTTP server");
            let _ = fs::write(CONFIG_FILE_NAME, serde_json::to_string_pretty(
                &Config {
                    server_ip: String::from("0.0.0.0"),
                    port: 8080
                }
            ).expect("Failed to serialize config")).expect("Failed to write config");
            panic!("Failed to read config");
        }
    };

    let config = match serde_json::from_str::<Config>(&config_raw) {
        Ok(c) => c,
        Err(err) => {
            panic!("Failed to deserialize config");
        }
    };

    return config;
}
