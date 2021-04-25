use std::error::Error;
use std::fmt;

/// Wrong input was passed.
#[derive(Debug)]
pub struct UnexpectedInputError {
    input: String,
}

impl UnexpectedInputError {
    pub (crate) fn new(input: impl Into<String>) -> UnexpectedInputError {
        return UnexpectedInputError {
            input: input.into(),
        }
    }
}

impl fmt::Display for UnexpectedInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected input `{}`", self.input)
    }
}

impl Error for UnexpectedInputError {}