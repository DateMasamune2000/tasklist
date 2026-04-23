mod migrations;
mod task;
mod config;

fn main() {
    let connection = sqlite::open(config::get_db_path()).unwrap();

    match connection.execute(migrations::TEST_QUERY) {
        Ok(_) => {},
        Err(_) => {
            println!("DB does not exist. Creating table...");
            connection.execute(migrations::CREATE_TABLE).unwrap();
			// TODO: temporary, remove this later
    		connection.execute(migrations::SAMPLE_ENTRIES).unwrap();
        }
    };

    let mut tasks: Vec<task::Task> = vec![];

    let query = "SELECT * FROM tasks ORDER BY id;";

    let _ = connection.iterate(query, |vals| {
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

		for task in tasks.iter() {
			println!("{}\t{}\t{}\t{}\t",
				task.id,
				task.project,
				task.task,
				match task.deadline {
					None => 0,
					Some(x) => x
				});
		}
    });
}
