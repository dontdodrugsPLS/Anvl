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
