use std::thread;
use std::sync::Arc;
use crate::tasks::read::{ProcfillConfig, read_yml};
use crate::utils::datautil;
use crate::pm::proc;

pub fn run(args: Vec<String>) {
    // Check if --start flag is provided to start tasks
    let start_tasks = args.contains(&"--start".to_string());
    
    if start_tasks {
        start_background_tasks();
    } else {
        list_existing_processes();
    }
}

fn start_background_tasks() {
    // Read configuration from YAML file
    let config = read_yml("procfil.yaml");
    
    let master_pid = std::process::id();
    println!("Starting processes in background. Master PID: {}", master_pid);
    
    // Create directories
    if let Err(e) = create_directories(&config) {
        eprintln!("Failed to create directories: {}", e);
        return;
    }
    
    let config_arc = Arc::new(config);
    let tasks = config_arc.commands.tasks.clone();
    
    // Spawn all tasks in detached threads
    for task in tasks {
        let config_clone = Arc::clone(&config_arc);
        thread::spawn(move || {
            match proc::run_task(&task, &config_clone, master_pid) {
                Ok(pid) => println!("Task '{}' started with PID: {}", task.name, pid),
                Err(e) => eprintln!("Failed to start task '{}': {}", task.name, e),
            }
        });
    }
    
    println!("All background tasks have been spawned. Exiting main process.");
}

fn list_existing_processes() {
    let config = ProcfillConfig {
        config_dir: "procfil/".to_string(),
        data_dir: "procfil/data".to_string(),
        log_dir: "procfil/logs".to_string(),
        commands: crate::tasks::read::Commands {
            options: crate::tasks::read::ProcfillOptions {
                save_output: false,
                run_parallel: false,
            },
            tasks: vec![],
        },
    };
    
    match datautil::list_processes(&config) {
        Ok(processes) => {
            if processes.is_empty() {
                println!("No processes found.");
            } else {
                println!("{:<10} {:<8} {:<15} {:<30} {:<10}", "PID", "MASTER", "NAME", "COMMAND", "STATUS");
                println!("{}", "-".repeat(75));
                
                for process in processes {
                    println!(
                        "{:<10} {:<8} {:<15} {:<30} {:<10}",
                        process.pid,
                        process.master_pid,
                        process.name,
                        format!("{} {}", process.command, process.args.join(" ")),
                        process.status
                    );
                }
            }
        }
        Err(e) => eprintln!("Failed to list processes: {}", e),
    }
}

fn create_directories(config: &ProcfillConfig) -> Result<(), std::io::Error> {
    use std::fs;
    
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