use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    repo: String,
    anvl_storage_path: String,
    always_push: bool,
}

impl Config {
    fn path() -> Result<PathBuf, String> {
        let mut path = dirs::config_dir().ok_or("could not determine config directory")?;

        path.push("anvl");
        path.push("config.json");

        Ok(path)
    }
}

impl Config {
    pub fn get() -> Result<Self, String> {
        let path = Self::path()?;
        let json =
            fs::read_to_string(&path).map_err(|e| format!("failed to read config file {e}"))?;

        serde_json::from_str(&json)
            .map_err(|e| format!("failed to parse config file {:?}: {}", path, e))
    }
}
