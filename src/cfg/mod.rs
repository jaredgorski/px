use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod config;
pub mod profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub config: config::Config,
    pub profile: profile::Profile,
}

impl Default for Cfg {
    fn default() -> Self {
        Self::new()
    }
}

impl Cfg {
    pub fn new() -> Cfg {
        let config = config::Config::new();
        let profile = profile::Profile::new();

        Cfg { config, profile }
    }
}

pub fn get_tog_cfg(profile_path: &str) -> Cfg {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(dir) => dir,
        _ => panic!(),
    };
    let tog_dirname: PathBuf = PathBuf::from(".tog");
    let tog_dir: PathBuf = [home_dir, tog_dirname].iter().collect();

    let config = match config::get_tog_rc(tog_dir.clone()) {
        Ok(config) => config,
        Err(error) => panic!(error),
    };

    let prof_dir: PathBuf = if profile_path == "default" {
        tog_dir.clone()
    } else {
        PathBuf::from(".")
    };

    let prof_path: String = if profile_path == "default" {
        config.profile.clone()
    } else {
        profile_path.to_string()
    };

    let profile = match profile::get_tog_pr(prof_dir, PathBuf::from(prof_path)) {
        Ok(profile) => profile,
        Err(error) => panic!(error),
    };

    let cfg: Cfg = Cfg { config, profile };

    cfg
}

pub fn default_cwd() -> String {
    ".".to_string()
}

pub fn default_empty_string() -> String {
    "".to_string()
}

pub fn default_false() -> bool {
    false
}

pub fn default_empty_vec_string() -> Vec<String> {
    Vec::new()
}
