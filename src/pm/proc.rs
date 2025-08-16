use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use crate::tasks::read::{ProcfillTask, ProcfillConfig};
use crate::utils::datautil::{ProcessInfo, save_process_info, save_process_output};

pub fn run_task(task: &ProcfillTask, config: &ProcfillConfig, master_pid: u32) -> Result<u32, anyhow::Error> {
    let mut cmd = Command::new(&task.command);
    
    cmd.current_dir(&task.dir);
    cmd.args(&task.args);
    
    // Detach the process to ensure it survives parent process exit
    #[cfg(unix)]
    {
        unsafe {
            cmd.pre_exec(|| {
                // Create a new session and process group
                libc::setsid();
                Ok(())
            });
        }
    }
    
    if config.commands.options.save_output {
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
    } else {
        // Redirect output to null to prevent zombie processes
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }
    
    let mut child = cmd.spawn()?;
    let pid = child.id();
    
    let process_info = ProcessInfo {
        name: task.name.clone(),
        command: task.command.clone(),
        args: task.args.clone(),
        dir: task.dir.clone(),
        pid,
        status: "running".to_string(),
        master_pid,
    };
    
    save_process_info(config, &process_info)?;
    
    if config.commands.options.save_output {
        let pid_clone = pid;
        let config_clone = config.clone();
        let task_name = task.name.clone();
        
        if let Some(stdout) = child.stdout.take() {
            let config_out = config_clone.clone();
            let name_out = task_name.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Err(e) = save_process_output(&config_out, pid_clone, &name_out, &line) {
                            eprintln!("Failed to save output for {}: {}", name_out, e);
                        }
                    }
                }
            });
        }
        
        if let Some(stderr) = child.stderr.take() {
            let name_err = task_name.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Err(e) = save_process_output(&config_clone, pid_clone, &name_err, &format!("ERROR: {}", line)) {
                            eprintln!("Failed to save error output for {}: {}", name_err, e);
                        }
                    }
                }
            });
        }
        
        println!("Output logging enabled for '{}' -> {}/{}_{}.log", task.name, config.log_dir, task.name, pid);
    }
    
    println!("Started process '{}' with PID: {}", task.name, pid);
    
    Ok(pid)
}

pub fn kill_process(pid: u32) -> Result<(), anyhow::Error> {
    use std::process::Command;
    
    #[cfg(unix)]
    {
        Command::new("kill")
            .arg(pid.to_string())
            .output()?;
    }
    
    #[cfg(windows)]
    {
        Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output()?;
    }
    
    println!("Killed process with PID: {}", pid);
    Ok(())
}