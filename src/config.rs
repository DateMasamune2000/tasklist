use dirs;
use std::path::PathBuf;

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
