// use std::process::Command;
// const INTERNAL_FILE_PATH: &str = "/var/lib/procfill/appdata";
// const LOG_FILE_PATH: &str = "/var/log/procfill/logs";
// const INTERNAL_DIRS: [&str; 2] = [INTERNAL_FILE_PATH, LOG_FILE_PATH];
//
// fn create_dir(dir_name: &str) {
//     log::debug!("Creating dir {}", dir_name);
//     Command::new("mkdir")
//         .arg("-p")
//         .arg(dir_name)
//         .spawn()
//         .unwrap();
//     log::debug!("Created dir {}", dir_name);
// }
//
// fn remove_dir(dir_name: &str) {
//     log::debug!("Removing dir {}", dir_name);
//     Command::new("rm").arg("-rf").arg(dir_name).spawn().unwrap();
//     log::debug!("Removed dir {}", dir_name);
// }
//
// pub fn create_dirs() {
//     for dir in INTERNAL_DIRS {
//         create_dir(dir);
//     }
// }
//
// pub fn remove_dirs() {
//     for dir in INTERNAL_DIRS {
//         remove_dir(dir)
//     }
// }
