use std::thread;
use std::sync::Arc;
use std::fs;

use crate::pm::proc;
use super::read::ProcfillConfig;

fn create_directories(config: &ProcfillConfig) -> Result<(), std::io::Error> {
    fs::create_dir_all(&config.config_dir)?;
    fs::create_dir_all(&config.data_dir)?;
    fs::create_dir_all(&config.log_dir)?;
    
    for task in &config.commands.tasks {
        if !task.dir.is_empty() {
            fs::create_dir_all(&task.dir)?;
        }
    }
    
    Ok(())
}

pub fn run(procfill_config: ProcfillConfig) {
    let master_pid = std::process::id();
    println!("Master process PID: {}", master_pid);
    
    if let Err(e) = create_directories(&procfill_config) {
        eprintln!("Failed to create directories: {}", e);
        return;
    }
    
    let config = Arc::new(procfill_config);
    let tasks = config.commands.tasks.clone();
    
    if config.commands.options.run_parallel {
        let mut handles = vec![];
        
        for task in tasks {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                match proc::run_task(&task, &config_clone, master_pid) {
                    Ok(pid) => println!("Task '{}' started with PID: {}", task.name, pid),
                    Err(e) => eprintln!("Failed to start task '{}': {}", task.name, e),
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        for task in tasks {
            match proc::run_task(&task, &config, master_pid) {
                Ok(pid) => println!("Task '{}' started with PID: {}", task.name, pid),
                Err(e) => eprintln!("Failed to start task '{}': {}", task.name, e),
            }
        }
    }
    
    println!("All tasks have been started.");
}