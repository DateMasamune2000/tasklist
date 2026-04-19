#[derive(Debug)]
pub struct Task {
	id: u64,
	project: String,
	task: String,
	deadline: Option<u64>,
}

#[derive(Default)]
pub struct TaskBuilder {
	id: Option<u64>,
	project: Option<String>,
	task: Option<String>,
	deadline: Option<Option<u64>>,
}

impl TaskBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_id(&mut self, id: u64) -> &mut Self {
		self.id = Some(id);
		self
	}

	pub fn set_task(&mut self, task: String) -> &mut Self {
		self.task = Some(task);
		self
	}

	pub fn set_project(&mut self, project: String) -> &mut Self {
		self.project = Some(project);
		self
	}

	pub fn set_deadline(&mut self, deadline: Option<u64>) -> &mut Self {
		self.deadline = Some(deadline);
		self
	}

	pub fn build(self) -> Result<Task, String> {
		Ok(Task {
			id: self.id.ok_or("missing id")?,
			task: self.task.ok_or("missing task")?,
			project: self.project.ok_or("missing project")?,
			deadline: self.deadline.ok_or("missing deadline")?,
		})
	}
}

