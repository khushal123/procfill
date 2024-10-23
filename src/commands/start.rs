use crate::{pm, utils};

pub fn run(args: Vec<String>) {
    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "--filename" => {
                let cmd_args = utils::fileutil::read_input_file(args.get(1).unwrap());
                pm::procutils::read_process(cmd_args);
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
