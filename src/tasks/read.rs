use std::{env, fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

// Use current serde derive macros
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")] // Handles kebab-case YAML keys automatically
pub struct ProcfillConfig {
    pub config_dir: String,
    pub data_dir: String,
    pub log_dir: String,
    pub commands: Commands,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcfillOptions {
    save_output: bool,
    run_parallel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    options: ProcfillOptions,
    tasks: Vec<ProcfillTask>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcfillTask {
    name: String,
    command: String,
    dir: String,
    #[serde(default)] // Make args optional
    args: Vec<String>,
}

impl ProcfillConfig {
    pub fn load(yml_file: &str) -> Self {
        let path: PathBuf = env::current_dir().unwrap().join(yml_file);
        println!("{}", path.to_string_lossy());
        let contents = fs::read_to_string(&path)
            .with_context(|| path.to_string_lossy().to_string())
            .unwrap();
        let config: Self = serde_yaml::from_str(&contents).unwrap();
        config
    }
}

pub fn read_yml(file_path: &str) -> ProcfillConfig {
    let config = ProcfillConfig::load(file_path);
    return config;
}
