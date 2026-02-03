use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    repo: String,
    anvl_storage_path: String,
    always_push: bool,
}
