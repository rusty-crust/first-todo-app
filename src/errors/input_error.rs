use std::num::ParseIntError;

#[derive(Debug)]
pub struct InputError {
    pub message: String,
}

impl From<ParseIntError> for InputError {
    fn from(value: ParseIntError) -> Self {
        InputError {
            message: value.to_string(),
        }
    }
}
