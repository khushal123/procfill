use std::{fs::File, io::Write, path::Path, process::Command};

const PID_STORAGE: &str = "pids.txt";
const COMMAND_FILE_PATH: &str = "command.txt";
pub const PORT_LIST: [i32; 9] = [5001, 5002, 5003, 5004, 5005, 5006, 5007, 5008, 5009];

pub fn stop_process(port: i32) -> i8 {
    let process = Command::new("lsof")
        .args(["-i", format!(":{port}").as_str()])
        .output()
        .expect("Failed to execute command");

    let stdlines = String::from_utf8_lossy(&process.stdout);

    let lines: Vec<&str> = stdlines.lines().collect();
    let mut pid: i32 = 0;
    if lines.len() == 0 {
        return 0;
    }
    for line in lines {
        if line.contains("node") {
            let linevec: Vec<&str> = line.split(' ').collect();
            pid = linevec[4].parse().unwrap();
        }
    }
    Command::new("kill")
        .args(["-9", port.to_string().as_str(), pid.to_string().as_str()])
        .output()
        .expect("Failed to execute command");
    return 1;
}

pub fn stop_all_processes() {
    for port in PORT_LIST {
        let result: i8 = stop_process(port);
        println!("{port} killd result {result}");
    }
}

pub fn create_command(port: i32) {
    println!("createing process");
    let output = Command::new("node")
        .args([
            "/home/ksc/dev/bun/server/index.js",
            port.to_string().as_str(),
        ])
        .spawn()
        .expect("Failed to execute command");

    let path = Path::new(PID_STORAGE);
    let mut file = File::options().append(true).open(path).unwrap();
    let _ = file.write(output.id().to_string().as_bytes());
    let _ = file.write("\n".as_bytes());
}
