use crate::{errors::todo_error::TodoError, todo::todo_store::TodoStore};

pub fn complete_todo(id: &usize, store: &TodoStore) -> Result<(), TodoError> {
    let mut todos = store.todos()?;

    for (i, todo) in todos.iter_mut().enumerate() {
        if i + 1 == *id {
            todo.complete();
            break;
        }
    }

    store.insert(&todos)?;

    println!("Todo completed! \n");

    Ok(())
}
