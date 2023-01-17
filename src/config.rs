use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub port: u16,
    pub files: &'static str,
    pub routes: &'static str
}

pub static mut CONFIG: Config = Config{
    port: 8080,
    files: "files",
    routes: "routes"
};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn load_config() {
    let config_str: String = std::fs::read_to_string("config.json").unwrap();
    
    let config_tmp: Config = serde_json::from_str::<Config>(string_to_static_str(config_str)).unwrap(); 

    unsafe {
        CONFIG = config_tmp;
    }
}