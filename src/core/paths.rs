use crate::core::config::Config;
use std::path::PathBuf;

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
