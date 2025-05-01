mod errors;
mod handlers;
mod todo;

use errors::input_error::InputError;
use handlers::{add::add_todo, list::list_todos};
use std::io;
use todo::{command::Command, todo::Todo, todo_store::TodoStore};

pub const ROOT_COMMAND: &str = "todo";

fn main() {
    println!("Hello, Todo!");
    let mut input = String::new();
    let mut store = TodoStore::init();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read the Input");

    let command = get_command(&input);

    if let Err(err) = command {
        return eprintln!("error: {}", err.message);
    }

    handle_command(&command.unwrap(), &mut store);
}

fn handle_command(command: &Command, store: &mut TodoStore) {
    match command {
        Command::ADD { body } => match add_todo(Todo::new(body.to_string()), store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },

        Command::LIST => match list_todos(store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },
    }
}

fn get_command(input: &str) -> Result<Command, InputError> {
    let mut input = input.trim().split(' ').into_iter();

    let first_word = input.next().ok_or(InputError {
        message: "Invalid command".to_string(),
    });

    if first_word.unwrap() != ROOT_COMMAND {
        return Err(InputError {
            message: "Invalid command".to_string(),
        });
    }

    let command_name = input.next().unwrap();

    match command_name {
        "add" => {
            let argument = &input.next();
            if argument.is_none() {
                return Err(InputError {
                    message: "Todo name is required".to_string(),
                });
            }
            Ok(Command::ADD {
                body: argument.unwrap().to_string(),
            })
        }
        "list" => Ok(Command::LIST),
        _ => Err(InputError {
            message: "Unkwnown command".to_string(),
        }),
    }
}
