use std::process::{Command, Output};

use crate::tasks::read::ProcfillTask;

fn run_program(procfil_task: &ProcfillTask) -> Result<Output, anyhow::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{} {}",
            procfil_task.command,
            procfil_task.args.join(" ")
        ))
        .output()
        .unwrap();
    Ok(output)
}

pub fn run_task(task: ProcfillTask) {
    let output = run_program(&task);
    match output {
        Ok(output) => println!("{}", output.status),
        Err(e) => println!("{}", e),
    }
}
