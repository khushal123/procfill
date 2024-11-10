use std::process::Command;
const INTERNAL_DIRS_KEYS: [&str; 2] = ["PROCFILL_LOG_DIR", "PROCFIL_DATA_DIR"];
const INTERNAL_DIRS: [&str; 2] = ["~/.procfil/logs", "~/.procfil/data"];

pub fn get_internal_dir_paths() -> Vec<String> {
    let mut internal_dirs: Vec<String> = vec![];

    //check log dir
    if let Ok(internal_dir) = std::env::var(INTERNAL_DIRS_KEYS[0]) {
        internal_dirs.push(internal_dir);
    } else {
        std::env::set_var(INTERNAL_DIRS_KEYS[0], INTERNAL_DIRS[1]);
        internal_dirs.push(INTERNAL_DIRS[0].to_string());
    }

    //check data dir
    if let Ok(internal_dir) = std::env::var(INTERNAL_DIRS_KEYS[1]) {
        internal_dirs.push(internal_dir);
    } else {
        std::env::set_var(INTERNAL_DIRS_KEYS[1], INTERNAL_DIRS[1]);
        internal_dirs.push(INTERNAL_DIRS[1].to_string());
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
