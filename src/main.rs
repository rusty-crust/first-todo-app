mod errors;
mod handlers;
mod todo;

use errors::input_error::InputError;
use handlers::{add::add_todo, complete::complete_todo, delete::delete_todo, list::list_todos};
use std::io::{self};
use todo::{command::Command, todo::Todo, todo_store::TodoStore};

pub const ROOT_COMMAND: &str = "todo";

fn main() {
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
        Command::Add(body) => match add_todo(Todo::new(body.to_string()), store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },

        Command::List => match list_todos(store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },

        Command::Complete(id) => match complete_todo(id, store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },

        Command::Delete(index) => match delete_todo(index, store) {
            Err(error) => eprintln!("{}", error.message),
            _ => (),
        },
    }
}

fn get_command(input: &str) -> Result<Command, InputError> {
    let mut input = input.trim().split(' ').into_iter();

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
    return input.next().ok_or_else(|| InputError {
        message: message.to_string(),
    });
}
