use crate::tasks;

pub fn run(args: Vec<String>) {
    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "--filename" => {
                let procfill_config = tasks::read::read_yml(&args.get(1).unwrap());
                tasks::task_runner::run(procfill_config);
            }
            _ => {
                panic!("invalid cli arg");
            }
        },
        None => {
            panic!("no cli args");
        }
    }
}
