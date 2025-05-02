use crate::{errors::todo_error::TodoError, todo::todo_store::TodoStore};

pub fn list_todos(store: &TodoStore) -> Result<(), TodoError> {
    for (i, todo) in store.todos()?.iter().enumerate() {
        println!(
            "[{}] — {} {}",
            i + 1,
            if todo.completed { "✅" } else { "🚫" },
            todo.name
        );
    }

    println!("\n");

    Ok(())
}
