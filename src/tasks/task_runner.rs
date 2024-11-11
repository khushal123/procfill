use crate::{pm::proc, utils::datautil};

use super::read::ProcfillConfig;

pub fn run(procfill_config: ProcfillConfig) {
    // let options = procfill_config.commands.options;
    let tasks = procfill_config.commands.tasks;
    for task in tasks {
        datautil::create_process(&task);
        proc::run_task(task);
    }
}
