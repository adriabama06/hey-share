use serde_derive::{Deserialize, Serialize};

use once_cell::sync::Lazy;


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub port: u16,
    pub files: String,
    pub routes: String,
    pub secret: String
}

impl Default for Config {
    fn default() -> Config {
        Config{
            port: 8080,
            files: "files".to_string(),
            routes: "routes".to_string(),
            secret: "secret".to_string()
        }
    }
}

pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);

pub fn load_config() {
    let config_str: String = std::fs::read_to_string("config.json").unwrap();
    
    let config_tmp: Config = serde_json::from_str::<Config>(&config_str).unwrap(); 

    unsafe {
        CONFIG.port = config_tmp.port;
        CONFIG.files = config_tmp.files;
        CONFIG.routes = config_tmp.routes;
        CONFIG.secret = config_tmp.secret;
    }
}