use crate::{
    errors::todo_error::TodoError,
    todo::{todo::Todo, todo_store::TodoStore},
};

pub fn delete_todo(index: &usize, store: &mut TodoStore) -> Result<(), TodoError> {
    let todos = store.todos()?;
    let todo_to_delete = store.get_by_sequential_index(index)?;

    let todos: Vec<Todo> = todos
        .into_iter()
        .filter(|todo| todo_to_delete.id != todo.id)
        .collect();

    store.insert(&todos)?;

    Ok(())
}
