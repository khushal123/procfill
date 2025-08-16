use std::{fs, path::PathBuf};
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::tasks::read::ProcfillConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub dir: String,
    pub pid: u32,
    pub status: String,
    pub master_pid: u32,
}

pub fn save_process_info(config: &ProcfillConfig, process_info: &ProcessInfo) -> Result<(), std::io::Error> {
    let data_dir = PathBuf::from(&config.data_dir);
    fs::create_dir_all(&data_dir)?;
    
    let pid_file = data_dir.join(format!("{}.json", process_info.pid));
    let json_data = serde_json::to_string_pretty(process_info)?;
    
    let mut file = fs::File::create(pid_file)?;
    file.write_all(json_data.as_bytes())?;
    
    Ok(())
}

pub fn save_process_output(config: &ProcfillConfig, pid: u32, task_name: &str, output: &str) -> Result<(), std::io::Error> {
    if !config.commands.options.save_output {
        return Ok(());
    }
    
    let log_dir = PathBuf::from(&config.log_dir);
    fs::create_dir_all(&log_dir)?;
    
    let output_file = log_dir.join(format!("{}_{}.log", task_name, pid));
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    writeln!(file, "[{}] {}", timestamp, output)?;
    Ok(())
}

pub fn list_processes(config: &ProcfillConfig) -> Result<Vec<ProcessInfo>, std::io::Error> {
    let data_dir = PathBuf::from(&config.data_dir);
    let mut processes = Vec::new();
    
    if !data_dir.exists() {
        return Ok(processes);
    }
    
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let contents = fs::read_to_string(&path)?;
            if let Ok(process_info) = serde_json::from_str::<ProcessInfo>(&contents) {
                processes.push(process_info);
            }
        }
    }
    
    Ok(processes)
}

pub fn remove_process_info(config: &ProcfillConfig, pid: u32) -> Result<(), std::io::Error> {
    let data_dir = PathBuf::from(&config.data_dir);
    let pid_file = data_dir.join(format!("{}.json", pid));
    
    if pid_file.exists() {
        fs::remove_file(pid_file)?;
    }
    
    Ok(())
}
