use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub remote:Option<String>,
    pub branch:Option<String>,
}

pub fn read_config() -> Config {
    let mut file = match File::open("config.json") {
        Ok(file) => file,
        Err(_) => {
            let default_config = Config {
                remote: None,
                branch: None,
            };
            return default_config;
        }
    };

    let mut contents = String::new();
    // get the contents of the file
    file.read_to_string(&mut contents)
        .expect("Failed to read the file");

    // deserialize the contents of the file which means we convert the content from string to an object(Config)
    let config: Config = serde_json::from_str(&contents)
        .expect("Failed to parse the file");

    config
}

