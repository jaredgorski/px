use crate::cfg::{default_empty_string, default_false};
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_empty_string")]
    pub profile: String,
    pub logging: LoggingCfg,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            profile: "".to_string(),
            logging: LoggingCfg::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoggingCfg {
    #[serde(default = "default_false")]
    pub sidebar: bool,
}

impl LoggingCfg {
    pub fn new() -> LoggingCfg {
        LoggingCfg { sidebar: false }
    }
}

pub fn get_tog_rc(tog_dir: PathBuf) -> Result<Config, Error> {
    let cfg_filename: PathBuf = PathBuf::from("tog.conf.yaml");
    let cfg_path: PathBuf = [tog_dir, cfg_filename].iter().collect();
    let mut cfg_file: File = File::open(cfg_path).expect("Unable to open config file");

    let mut cfg_file_str = String::new();
    cfg_file
        .read_to_string(&mut cfg_file_str)
        .expect("Unable to read config file");
    let config: Result<Config, Error> = serde_yaml::from_str(&cfg_file_str);

    config
}
