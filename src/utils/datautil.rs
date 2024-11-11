use rusqlite::Connection;
use std::path;

pub fn create_tables(data_dir: &str) {
    let path = path::Path::new(data_dir)
        .join("procfil.db");
    println!("{}", path.to_string_lossy());
    let conn: Connection = Connection::open(&path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS procfill (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            output TEXT NOT NULL,
            pid INTEGER NOT NULL,
            group_id DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .unwrap();
}
