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
    pub fn get_runner(&self, arguments: Vec<String>) {
        match self {
            Action::Help => {
                println!("Help {}", arguments.len());
            }
            Action::Start => {
                commands::start::run(arguments);
            }
            Action::Log => {
                commands::kill::run(arguments);
            }
            Action::Kill => {
                commands::kill::run(arguments);
            }
            Action::Pause => {
                println!("Pause {}", arguments.len());
            }
            Action::LS => {
                println!("LS {}", arguments.len());
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
