use crate::pm::proc;
use crate::tasks::read::ProcfillConfig;
use crate::utils::datautil;

pub fn run(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Error: Please provide a PID to kill");
        return;
    }
    
    let pid: u32 = match args[0].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Error: Invalid PID '{}'", args[0]);
            return;
        }
    };
    
    match proc::kill_process(pid) {
        Ok(_) => {
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
            
            let _ = datautil::remove_process_info(&config, pid);
            println!("Successfully killed process {}", pid);
        }
        Err(e) => eprintln!("Failed to kill process {}: {}", pid, e),
    }
}