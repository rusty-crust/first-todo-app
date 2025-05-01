use crate::{errors::todo_error::TodoError, todo::todo_store::TodoStore};

pub fn list_todos(store: &TodoStore) -> Result<(), TodoError> {
    // if store.is_empty() {
    //     return println!("Nothing to do at the moment");
    // }

    for (i, todo) in store.todos()?.iter().enumerate() {
        println!("[{:1}] — {:2}", i + 1, todo.name);
    }

    Ok(())
}
