use dirs;
use std::path::PathBuf;
use std::path::Path;

mod migrations;
mod task;

fn main() {
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
        std::fs::create_dir(&path);
    }

    path.push("tasks.db");

    let connection = sqlite::open(path).unwrap();

    match connection.execute(migrations::TEST_QUERY) {
        Ok(_) => {},
        Err(_) => {
            println!("DB does not exist. Creating table...");
            connection.execute(migrations::CREATE_TABLE).unwrap();
        }
    };

    connection.execute(migrations::SAMPLE_ENTRIES).unwrap();

    let mut tasks: Vec<task::Task> = vec![];

    let query = "SELECT * FROM tasks;";

    let result = connection.iterate(query, |vals| {
        let mut tb = task::TaskBuilder::new();

        for &(key, value) in vals.iter() {
            match key {
                "id" => tb.set_id(value
                    .unwrap()
                    .parse::<u64>()
                    .expect("u64 id")),

                "project" => tb.set_project(
                    String::from(value.unwrap())),

                "task" => tb.set_task(
                    String::from(value.unwrap())),

                "deadline" => tb.set_deadline(match value {
                    None => None,
                    Some(x) => Some(x.parse::<u64>()
                        .expect("u64 timestamp for deadline"))
                }),

                &_ => panic!("unsupported column found in table")
            };

        }
        tasks.push(tb.build().expect("all required columns done"));
        true
    });

    println!("{:?}", tasks);
}
