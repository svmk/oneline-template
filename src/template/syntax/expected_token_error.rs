use std::error::Error;
use std::fmt;

/// Wrong input was passed when some token is expected.
#[derive(Debug)]
pub struct ExpectedTokenError {
    token: String,
    input: String,
}

impl ExpectedTokenError {
    pub (crate) fn new(token: impl Into<String>, input: impl Into<String>) -> ExpectedTokenError {
        return ExpectedTokenError {
            token: token.into(),
            input: input.into(),
        }
    }
}

impl fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expected token `{}` in `{}`", self.token, self.input)
    }
}

impl Error for ExpectedTokenError {}