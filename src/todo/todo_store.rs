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
}
