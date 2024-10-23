mod extract;
use extract::Action;
mod commands;
mod utils;
mod pm;
// mod dirutil;

// mod procutils;
// mod start;

fn main() {
    // dirutil::create_dirs();
    // procutils::stop_all_processes();
    // for port in procutils::PORT_LIST {
    //     println!("Starting process on port {}", port);
    //     // procutils::create_command(port)
    // }
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
    action.get_runner(arguments);
}
