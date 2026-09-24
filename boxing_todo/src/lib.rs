mod err;

use std::error::Error;

use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = std::fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let parsed = json::parse(&content).map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        if parsed["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let tasks = parsed["tasks"]
            .members()
            .map(|t| Task {
                id: t["id"].as_u32().unwrap_or_default(),
                description: t["description"].as_str().unwrap_or_default().to_owned(),
                level: t["level"].as_u32().unwrap_or_default(),
            })
            .collect();

        Ok(TodoList {
            title: parsed["title"].as_str().unwrap_or_default().to_owned(),
            tasks,
        })
    }
}
