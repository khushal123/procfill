use crate::pm::proc;

use super::read::ProcfillConfig;

pub fn run(procfill_config: ProcfillConfig) {
    // let options = procfill_config.commands.options;
    let tasks = procfill_config.commands.tasks;
    for task in tasks {
        proc::run_task(task);
    }
}
