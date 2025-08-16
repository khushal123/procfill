use crate::commands;

pub enum Action {
    Help,
    Start,
    Log,
    Kill,
    Pause,
    LS,
    Error,
}
impl Action {
    pub fn start_runner(&self, arguments: Vec<String>) {
        match self {
            Action::Help => {
                let help_string = "
                    --help print all options of the command
                    start -f filename -> start the processes in the given filename
                    log -> show the logs
                    kill -> kill the process
                    pause -> pause the process
                    ls -> list all processes
                ";
                println!("{}", help_string);
            }
            Action::Start => {
                commands::start::run(arguments);
            }
            Action::Log => {
                commands::log::run(arguments);
            }
            Action::Kill => {
                commands::kill::run(arguments);
            }
            Action::Pause => {
                commands::pause::run(arguments);
            }
            Action::LS => {
                commands::ls::run(arguments);
            }
            Action::Error => {
                println!("Error {}", arguments.len());
            }
        }
    }
}

pub fn parse_command(cmd: &str) -> Action {
    let action = match cmd {
        "--help" => Action::Help,
        "start" => Action::Start,
        "log" => Action::Log,
        "kill" => Action::Kill,
        "pause" => Action::Pause,
        "ls" => Action::LS,
        _ => Action::Error,
    };
    return action;
}
