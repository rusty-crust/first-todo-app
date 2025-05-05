use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
};

use crate::errors::todo_error::TodoError;

use super::todo::Todo;

pub const TODO_PATH: &str = "src/todo/todos.json";

#[derive(Debug)]
pub struct TodoStore {}

impl TodoStore {
    pub fn init() -> TodoStore {
        TodoStore {}
    }

    pub fn get_by_sequential_index(&self, index: &usize) -> Result<Todo, TodoError> {
        let todos = self.todos()?;

        for (i, todo) in todos.iter().enumerate() {
            if i == index - 1 {
                return Ok(todo.clone());
            }
        }

        Err(TodoError {
            message: "Not Found Todo".to_string(),
        })
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), TodoError> {
        let mut todos = self.todos()?;

        todos.push(todo);

        let file = OpenOptions::new().write(true).open(TODO_PATH)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &todos)?;
        writer.flush()?;

        Ok(())
    }

    pub fn todos(&self) -> Result<Vec<Todo>, TodoError> {
        let file = File::open(TODO_PATH)?;
        let reader = BufReader::new(file);
        let todos: Vec<Todo> = serde_json::from_reader(reader)?;
        Ok(todos)
    }

    pub fn insert(&self, todos: &Vec<Todo>) -> Result<(), TodoError> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .read(true)
            .open(TODO_PATH)?;
        let mut writer = BufWriter::new(file);
        writer.flush()?;

        serde_json::to_writer(&mut writer, &todos)?;
        writer.flush()?;

        Ok(())
    }
}
