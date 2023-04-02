use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}
