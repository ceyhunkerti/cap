pub mod resume;

use resume::Resume;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub template: String,
    pub resumes: HashMap<String, Resume>,
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
    pub fn resumes(&self) -> Vec<(bool, &String)> {
        self.resumes
            .iter()
            .map(|(k, v)| {
                (
                    self.resumes.keys().len() == 1 || v.default.unwrap_or(false),
                    k,
                )
            })
            .collect::<Vec<(bool, &String)>>()
    }
    pub fn resume(&self, name: &str) -> Option<&Resume> {
        self.resumes.get(name)
    }
}
