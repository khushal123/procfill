use std::process::Command;

use crate::tasks::read::ProcfillTask;

fn run_program(program: String, args: Vec<String>) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", program, args.join(" ")))
        .output()
        .unwrap();

    if output.status.success() {
        println!("Program ran successfully");
    } else {
        println!("Program failed with exit code: {}", output.status);
    }
}

pub fn run_task(task: ProcfillTask) {
    run_program(task.command, task.args);
}
