use std::{path, process::Command};
const INTERNAL_DIRS_KEYS: [&str; 2] = ["PROCFILL_LOG_DIR", "PROCFIL_DATA_DIR"];
const INTERNAL_DIRS: [&str; 2] = ["/.procfil/logs", "/.procfil/data"];

pub fn get_internal_dir_paths() -> Vec<String> {
    let mut internal_dirs: Vec<String> = vec![];
    let mut home_dir = get_home_dir();
    home_dir.push(path::MAIN_SEPARATOR);

    //check log dir
    if let Ok(internal_dir) = std::env::var(INTERNAL_DIRS_KEYS[0]) {
        home_dir.push_str(internal_dir.as_str());
        internal_dirs.push(home_dir);
    } else {
        std::env::set_var(INTERNAL_DIRS_KEYS[0], INTERNAL_DIRS[1]);
        home_dir.push_str(INTERNAL_DIRS[0]);
        internal_dirs.push(home_dir);
    }

    let mut home_dir = get_home_dir();
    //check data dir
    if let Ok(internal_dir) = std::env::var(INTERNAL_DIRS_KEYS[1]) {
        home_dir.push_str(internal_dir.as_str());
        internal_dirs.push(home_dir);
    } else {
        std::env::set_var(INTERNAL_DIRS_KEYS[1], INTERNAL_DIRS[1]);
        home_dir.push_str(INTERNAL_DIRS[1]);
        internal_dirs.push(home_dir);
    }

    return internal_dirs;
}
//
fn create_dir(dir_name: &str) {
    log::debug!("Creating dir {}", dir_name);
    Command::new("sh")
        .arg("-c")
        .arg(format!("mkdir -p {}", dir_name))
        .status()
        .unwrap();
    log::debug!("Created dir {}", dir_name);
}

fn remove_dir(dir_name: &str) {
    log::debug!("Removing dir {}", dir_name);
    Command::new("sh")
        .arg("-c")
        .arg(format!("mkdir -p {}", dir_name))
        .arg(dir_name)
        .status()
        .unwrap();
    log::debug!("Removed dir {}", dir_name);
}

pub fn create_dirs(directories: &Vec<String>) {
    for dir in directories {
        create_dir(dir.as_str());
    }
}

pub fn remove_dirs(directories: &Vec<String>) {
    for dir in directories {
        remove_dir(dir.as_str());
    }
}

pub fn get_home_dir() -> String {
    let home_dir = std::env::var("HOME").unwrap();
    return home_dir;
}
