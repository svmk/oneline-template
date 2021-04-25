use crate::template::syntax::expected_token_error::ExpectedTokenError;
use crate::template::syntax::invalid_argument_template::InvalidArgumentTemplate;
use crate::template::syntax::unexpected_input_error::UnexpectedInputError;

use std::num::ParseIntError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// Template format parsing error.
pub enum SyntaxParseError {
    /// Parsing integer error.
    ParseInt(ParseIntError),
    /// Function name contains invalid chars.
    WrongFunctionName,
    /// Field name contains invalid chars.
    WrongFieldName,
    /// Wrong input was passed when some token is expected.
    ExpectedToken(ExpectedTokenError),
    /// Wrong input was passed.
    UnexpectedInput(UnexpectedInputError),
    /// Template is empty. Nothing to format.
    TemplateIsEmpty,
    /// Unable to parse argument passed into function.
    InvalidArgumentTemplate(InvalidArgumentTemplate),
}

impl fmt::Display for SyntaxParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxParseError::ParseInt(ref error) => {
                write!(f, "{}", error)
            },
            SyntaxParseError::WrongFunctionName => {
                write!(f, "Wrong function name")
            },
            SyntaxParseError::WrongFieldName => {
                write!(f, "Wrong field name")
            },
            SyntaxParseError::ExpectedToken(ref error) => {
                write!(f, "{}", error)
            },
            SyntaxParseError::UnexpectedInput(ref error) => {
                write!(f, "{}", error)
            },
            SyntaxParseError::TemplateIsEmpty => {
                write!(f, "Template is empty")
            },
            SyntaxParseError::InvalidArgumentTemplate(ref error) => {
                write!(f, "{}", error)
            },
        }
    }
}

impl From<ParseIntError> for SyntaxParseError {
    fn from(error: ParseIntError) -> Self {
        return SyntaxParseError::ParseInt(error);
    }
}

impl From<ExpectedTokenError> for SyntaxParseError {
    fn from(error: ExpectedTokenError) -> Self {
        return SyntaxParseError::ExpectedToken(error);
    }
}

impl From<UnexpectedInputError> for SyntaxParseError {
    fn from(error: UnexpectedInputError) -> Self {
        return SyntaxParseError::UnexpectedInput(error);
    }
}

impl From<InvalidArgumentTemplate> for SyntaxParseError {
    fn from(error: InvalidArgumentTemplate) -> Self {
        return SyntaxParseError::InvalidArgumentTemplate(error);
    }
}

impl Error for SyntaxParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SyntaxParseError::ParseInt(ref error) => {
                Some(error)
            },
            SyntaxParseError::WrongFunctionName => {
                None
            },
            SyntaxParseError::WrongFieldName => {
                None
            },
            SyntaxParseError::ExpectedToken(ref error) => {
                Some(error)
            },
            SyntaxParseError::UnexpectedInput(ref error) => {
                Some(error)
            },
            SyntaxParseError::TemplateIsEmpty => {
                None
            },
            SyntaxParseError::InvalidArgumentTemplate(ref error) => {
                Some(error)
            },
        }
    }
}