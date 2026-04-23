use dirs;
use std::path::PathBuf;

use crate::migrations;

pub fn get_db_path() -> PathBuf {
    let mut path = match dirs::config_dir() {
        None => {
            println!("Cannot find config directory. Using workdir.");
            PathBuf::new()
        },

        Some(x) => x
    };

    path.push("tasklist");

    if !path.exists() {
        println!("path does not exist");
        let _ = std::fs::create_dir(&path);
    }

    path.push("tasks.db");

	return path;
}

pub fn create_table_if_absent(connection: &sqlite::Connection) {
    match connection.execute(migrations::TEST_QUERY) {
        Ok(_) => {},
        Err(_) => {
            println!("DB does not exist. Creating table...");
            connection.execute(migrations::CREATE_TABLE).unwrap();
			// TODO: temporary, remove this later
    		connection.execute(migrations::SAMPLE_ENTRIES).unwrap();
        }
    };
}
