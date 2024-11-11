mod bootstrap;
mod commands;
mod extract;
mod pm;
mod tasks;
mod utils;
use extract::Action;

// mod procutils;
// mod start;

fn main() {
    bootstrap::start();
    let args: Vec<String> = std::env::args().collect();
    run(args);
}

fn run(args: Vec<String>) {
    if args.len() == 1 {
        panic!("no cli args");
    }
    let index = 1;
    let action: Action = extract::parse_command(&args[index]);
    let index = 2;
    let arguments = args[index..].to_vec();
    action.start_runner(arguments);
}
