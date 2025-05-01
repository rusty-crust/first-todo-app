use crate::errors::todo_error::TodoError;
use crate::todo::todo::Todo;
use crate::todo::todo_store::TodoStore;

pub fn add_todo(todo: Todo, store: &mut TodoStore) -> Result<(), TodoError> {
    store.add(todo)?;

    Ok(())
}
