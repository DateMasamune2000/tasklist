pub const CREATE_TABLE: &str = "
CREATE TABLE tasks(id INTEGER PRIMARY KEY, project TEXT, task TEXT, deadline INTEGER);
";

pub const SAMPLE_ENTRIES: &str = "
INSERT INTO tasks (project, task, deadline) VALUES ('dank memes', 'make dank meme', 420);
INSERT INTO tasks (project, task, deadline) VALUES ('dank memes', 'get cancelled', 67);
";

pub const TEST_QUERY: &str = "SELECT COUNT(*) FROM tasks;";
