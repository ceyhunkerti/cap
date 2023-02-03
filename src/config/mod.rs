pub mod resume;

use anyhow::Result;
use resume::Resume;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub template: String,
    pub resumes: HashMap<String, Resume>,
}

impl Config {
    pub fn new(config_file: Option<String>) -> Result<Config, anyhow::Error> {
        let default = String::from("config.yml");
        let resolved_config_file = match config_file {
            Some(f) => f,
            None => {
                info!("config file not given checking `CAP_HOME` environment variable...");
                match env::var("CAP_HOME") {
                    Ok(c) => c,
                    _ => {
                        warn!("no config file found using default ./{}", default);
                        default
                    }
                }
            }
        };
        let f = std::fs::File::open(resolved_config_file)?;
        let config = serde_yaml::from_reader(f)?;
        Ok(config)
    }
    pub fn resumes(&self) -> Vec<(bool, &String)> {
        let mut result = self
            .resumes
            .iter()
            .map(|(k, v)| {
                (
                    self.resumes.keys().len() == 1 || v.default.unwrap_or(false),
                    k,
                )
            })
            .collect::<Vec<(bool, &String)>>();
        result.sort_by_key(|k| k.1);
        result
    }
    pub fn resume(&self, name: &str) -> Option<&Resume> {
        self.resumes.get(name)
    }
}
