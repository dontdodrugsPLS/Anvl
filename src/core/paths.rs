use crate::core::config::Config;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectPaths {
    pub root: PathBuf,
    pub lockfile: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CachePaths {
    pub storage_root: PathBuf,
    pub repo_dir: PathBuf,
}

pub fn resolve_project_paths() -> Result<ProjectPaths, String> {
    let mut current_path =
        std::env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;

    loop {
        let lockfile = current_path.join("anvl.lock.json");
        if lockfile.is_file() {
            return Ok(ProjectPaths {
                root: current_path,
                lockfile: lockfile,
            });
        }
        if !current_path.pop() {
            break;
        }
    }
    Err(
        "not inside an Anvl project (anvl.lock.json not found in directory and all his parents)"
            .to_string(),
    )
}

pub fn get_cache_paths() -> Result<CachePaths, String> {
    let cfg = Config::get()?;

    fs::create_dir_all(&cfg.anvl_storage_path).map_err(|e| {
        format!(
            "failed to create '{}': {e}",
            cfg.anvl_storage_path.display()
        )
    })?;

    let repo_dir = cfg.anvl_storage_path.join("repo");

    Ok(CachePaths {
        storage_root: cfg.anvl_storage_path,
        repo_dir: repo_dir,
    })
}
