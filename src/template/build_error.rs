use crate::template::syntax::syntax_parse_error::SyntaxParseError as ParseError;
use crate::template::argument_types_differ_error::ArgumentTypesDifferError;
use std::error::Error;
use std::fmt;

/// Error reason when building template.
#[derive(Debug)]
pub enum BuildError {
    /// Error when parsing template string.
    Parse(ParseError),
    /// Error when function not registered withing template engine.
    FunctionNotFound(String),
    /// Error when number of arguments that passed into function is differ from number of required arguments.
    ArgumentsLengthDiffer(String),
    /// Error when passed argument type is differ from required. 
    ArgumentTypeNotMatch(ArgumentTypesDifferError),
}

impl From<ParseError> for BuildError {
    fn from(error: ParseError) -> Self {
        return BuildError::Parse(error);
    }
}

impl From<ArgumentTypesDifferError> for BuildError {
    fn from(error: ArgumentTypesDifferError) -> Self {
        return BuildError::ArgumentTypeNotMatch(error);
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Parse(ref error) => {
                write!(f, "{}", error)
            },
            BuildError::FunctionNotFound(ref function_name) => {
                write!(f, "Function with name `{}` not found", function_name)
            },
            BuildError::ArgumentsLengthDiffer(ref function_name) => {
                write!(f, "Into function `{}` passed arguments with wrong length", function_name)
            },
            BuildError::ArgumentTypeNotMatch(ref error) => {
                write!(f, "Argument with index `{}` at function `{}` has wrong type", error.get_argument_position(), error.get_function_name())
            },
        }
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildError::Parse(ref error) => {
                Some(error)
            },
            BuildError::FunctionNotFound(..) => {
                None
            },
            BuildError::ArgumentsLengthDiffer(..) => {
                None
            },
            BuildError::ArgumentTypeNotMatch(..) => {
                None
            },
        }
    }
}