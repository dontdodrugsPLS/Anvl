use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleMeta {
    pub name: String,

    #[serde(default)]
    pub desc: String,

    #[serde(default)]
    pub provides: Vec<ProvidesItem>,

    #[serde(default)]
    pub includes: String,

    #[serde(default)]
    pub deps: Vec<String>,

    #[serde(default)]
    pub public_headers: Vec<String>,

    #[serde(default)]
    pub private_headers: Vec<String>,

    #[serde(default)]
    pub src: Vec<String>,

    #[serde(default)]
    pub src_tests: Vec<String>,

    #[serde(default)]
    pub ld_flags: Vec<String>,

    #[serde(default)]
    pub extern_fn_calls: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProvidesItem {
    #[serde(default)]
    pub func: String,

    #[serde(default)]
    pub desc: String,
}
