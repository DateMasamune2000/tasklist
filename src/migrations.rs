pub const CREATE_TABLE: &str = "
CREATE TABLE tasks(id INTEGER PRIMARY KEY, project TEXT, task TEXT, deadline INTEGER);
INSERT INTO tasks (project, task, deadline) VALUES ('dank memes', 'make dank meme', 420);
INSERT INTO tasks (project, task, deadline) VALUES ('dank memes', 'get cancelled', 67);
";

