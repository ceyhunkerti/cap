pub mod resume;

use resume::Resume;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    resumes: HashMap<String, Resume>,
}

impl Config {
    pub fn new(config_file: Option<String>) -> Self {
        let default = String::from("~/.config/config.yml");
        let resolved_config_file = match config_file {
            Some(f) => f,
            None => env::var("CAP_HOME").unwrap_or(default),
        };
        let f = std::fs::File::open(resolved_config_file).expect("Could not open file.");
        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(f);
        config.expect("Unable to read config file")
    }
    pub fn resume_list(&self) -> Vec<String> {
        self.resumes.keys().cloned().collect::<Vec<String>>()
    }
    pub fn resume(&mut self, name: &str) -> Option<&mut Resume> {
        self.resumes.get_mut(name)
    }
}
