pub fn run(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Error: Please provide a PID to pause");
        return;
    }
    
    let pid: u32 = match args[0].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Error: Invalid PID '{}'", args[0]);
            return;
        }
    };
    
    #[cfg(unix)]
    {
        use std::process::Command;
        match Command::new("kill")
            .args(&["-STOP", &pid.to_string()])
            .output()
        {
            Ok(_) => println!("Process {} has been paused", pid),
            Err(e) => eprintln!("Failed to pause process {}: {}", pid, e),
        }
    }
    
    #[cfg(not(unix))]
    {
        eprintln!("Pause functionality is only available on Unix systems");
    }
}