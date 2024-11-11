use rusqlite::Connection;
use std::path;

use crate::tasks::read::ProcfillTask;
use crate::utils;
pub fn create_tables(data_dir: &str) {
    let path = path::Path::new(data_dir).join("procfil.db");
    println!("{}", path.to_string_lossy());
    let conn: Connection = Connection::open(&path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS procfill (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            output TEXT NOT NULL,
            status TEXT NOT NULL,
            pid INTEGER NOT NULL,
            group_id DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .unwrap();
}

pub fn create_process(procfil_task: &ProcfillTask) {
    let internal_dirs = utils::dirutil::get_internal_dir_paths();
    let path = path::Path::new(&internal_dirs[1]).join("procfil.db");
    let conn: Connection = Connection::open(&path).unwrap();
    let insert = "INSERT INTO procfill (name, command, output, status) VALUES (?, ?, ?, ?, ?)";
    conn.execute(
        &insert,
        (
            &procfil_task.name,
            &procfil_task.command,
            &procfil_task.output,
            &procfil_task.status,
            &procfil_task.pid,
        ),
    )
    .unwrap();
}

pub fn update_process(procfil_task: &ProcfillTask) {
    let internal_dirs = utils::dirutil::get_internal_dir_paths();
    let path = path::Path::new(&internal_dirs[1]).join("procfil.db");
    let conn: Connection = Connection::open(&path).unwrap();
    let insert =
        "UPDATE procfill SET name = ?, command = ?, output = ?, status = ?, pid = ? WHERE id = ?";
    conn.execute(
        &insert,
        (
            &procfil_task.name,
            &procfil_task.command,
            &procfil_task.output,
            &procfil_task.status,
            &procfil_task.pid,
        ),
    )
    .unwrap();
}
