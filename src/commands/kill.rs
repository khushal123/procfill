use crate::{pm, utils};

pub fn run(args: Vec<String>) {
    let pid = args[0].parse().unwrap();
    pm::procutils::kill_process(pid);
}
