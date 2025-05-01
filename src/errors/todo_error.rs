use serde_json::Error as SerderJsonError;
use std::io::Error;

#[derive(Debug)]
pub struct TodoError {
    pub message: String,
}

impl From<Error> for TodoError {
    fn from(value: Error) -> Self {
        TodoError {
            message: value.to_string(),
        }
    }
}

impl From<SerderJsonError> for TodoError {
    fn from(value: SerderJsonError) -> Self {
        TodoError {
            message: value.to_string(),
        }
    }
}
