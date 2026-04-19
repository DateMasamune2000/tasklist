mod migrations;
mod task;

fn main() {
	let connection = sqlite::open(":memory:").unwrap();

	connection.execute(migrations::CREATE_TABLE).unwrap();

	let mut tasks: Vec<task::Task> = vec![];

	let query = "SELECT * FROM tasks;";
	connection.iterate(query, |vals| {
		let mut tb = task::TaskBuilder::new();

		for &(key, value) in vals.iter() {
			match key {
				"id" => tb.set_id(value.unwrap().parse::<u64>().expect("u64 id")),
				"project" => tb.set_project(String::from(value.unwrap())),
				"task" => tb.set_task(String::from(value.unwrap())),
				"deadline" => tb.set_deadline(match value {
					None => None,
					Some(x) => Some(x.parse::<u64>().expect("u64 timestamp for deadline"))
				}),
				&_ => panic!("unsupported column found in table")
			};

		}
		tasks.push(tb.build().expect("all required columns done"));
		true
	})
	.unwrap();

	println!("{:?}", tasks);
}
