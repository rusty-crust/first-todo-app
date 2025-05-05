mod errors;
mod handlers;
mod todo;

use errors::input_error::InputError;
use handlers::{add::add_todo, complete::complete_todo, delete::delete_todo, list::list_todos};
use std::io::{self};
use todo::{command::Command, todo::Todo, todo_store::TodoStore};

pub const ROOT_COMMAND: &str = "todo";

#[test]
fn test_extract_command() {
    assert_eq!(true, true);
}

fn main() {
    let message = "Temp temperature today is:";
    let x = [message; 100];
    println!("{}: {}", x[0], x[1]);

    println!("Hello, Todo!");
    let mut input = String::new();
    let mut store = TodoStore::init();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let command = match get_command(&input) {
            Ok(cmd) => cmd,
            Err(error) => {
                eprintln!("{}", error.message);
                continue;
            }
        };

        handle_command(&command, &mut store);
    }
}

fn handle_command(command: &Command, store: &mut TodoStore) {
    match command {
        Command::Add(body) => {
            if let Err(error) = add_todo(Todo::new(body.to_string()), store) {
                eprintln!("{}", error.message);
            }
        }
        Command::List => {
            if let Err(error) = list_todos(store) {
                eprintln!("{}", error.message)
            }
        }

        Command::Complete(id) => {
            if let Err(error) = complete_todo(id, store) {
                eprintln!("{}", error.message);
            }
        }

        Command::Delete(index) => {
            if let Err(e) = delete_todo(index, store) {
                eprintln!("{}", e.message)
            }
        }
    }
}

fn get_command(input: &str) -> Result<Command, InputError> {
    let mut input = input.trim().split(' ');

    let first_word = input.next().ok_or(InputError {
        message: "Invalid command".to_string(),
    })?;

    if first_word != ROOT_COMMAND {
        return Err(InputError {
            message: "Invalid command".to_string(),
        });
    }

    let command_name = input.next().ok_or(InputError {
        message: "Arguments are not provided".into(),
    })?;

    match command_name {
        "add" => {
            let argument = input.next().ok_or_else(|| InputError {
                message: "Todo name is required".to_string(),
            })?;
            Ok(Command::Add(argument.to_string()))
        }
        "list" => Ok(Command::List),
        "complete" => {
            let argument = input.next().ok_or_else(|| InputError {
                message: "Todo name is required".to_string(),
            })?;

            Ok(Command::Complete(argument.parse()?))
        }
        "delete" => {
            let argument = parse_arguments(&mut input, "Please provde an ID to delete")?;
            Ok(Command::Delete(argument.parse()?))
        }
        _ => Err(InputError {
            message: "Unkwnown command".to_string(),
        }),
    }
}

fn parse_arguments<'a, I>(input: &mut I, message: &str) -> Result<&'a str, InputError>
where
    I: Iterator<Item = &'a str>,
{
    input.next().ok_or_else(|| InputError {
        message: message.to_string(),
    })
}
