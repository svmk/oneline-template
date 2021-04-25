use crate::function_executor::FunctionError;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;

/// Template execution error.
#[derive(Debug)]
pub enum TemplateError {
    /// Field in path not found.
    PathNotFound(String),
    /// Path contains sequence.
    PathContainsSeq(String),
    /// Path contains map.
    PathContainsMap(String),
    /// Path contains unknown type.
    PathContainsUnknownType(String),
    /// Path contains error type.
    PathContainsUnknownErrorType(String),
    /// Function not found.
    FunctionNotFound(String),
    /// Function execution error.
    FunctionError(FunctionError),
    /// Structure serialization error.
    SerializationError,
    /// Unable convert boolean to string.
    UnableConvertBoolToString,
    /// Unable convert option to string.
    UnableConvertOptionToString,
    /// Unable convert float to string.
    UnableConvertFloatToString,
    /// Unable convert `vec<u8>` to utf-8 string.
    VecToUtf8ConvertationError(FromUtf8Error),
}

impl From<FunctionError> for TemplateError {
    fn from(error: FunctionError) -> Self {
        return TemplateError::FunctionError(error);
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::PathNotFound(ref path) => {
                write!(f, "Path not found: `{}`", path)
            },
            TemplateError::PathContainsSeq(ref path) => {
                write!(f, "Path `{}` contains sequence", path)
            },
            TemplateError::PathContainsMap(ref path) => {
                write!(f, "Path `{}` contains map", path)
            },
            TemplateError::PathContainsUnknownType(ref path) => {
                write!(f, "Path `{}` contains unknown type", path)
            },
            TemplateError::PathContainsUnknownErrorType(ref path) => {
                write!(f, "Path `{}` contains unknown error type", path)
            },
            TemplateError::FunctionNotFound(ref function_name) => {
                write!(f, "Function `{}` not found", function_name)
            },
            TemplateError::FunctionError(ref error) => {
                write!(f, "{}", error)
            },
            TemplateError::SerializationError => {
                write!(f, "Error while serialization")
            },
            TemplateError::UnableConvertBoolToString => {
                write!(f, "Convertation bool to string is not supported")
            },
            TemplateError::UnableConvertOptionToString => {
                write!(f, "Convertation option to string is not supported")
            },
            TemplateError::UnableConvertFloatToString => {
                write!(f, "Convertation float to string is not supported")
            },
            TemplateError::VecToUtf8ConvertationError(ref error) => {
                write!(f, "Error while converting vec to string: {}", error)
            },
        }
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TemplateError::PathNotFound(..) => {
                return None;
            },
            TemplateError::PathContainsSeq(..) => {
                return None;
            },
            TemplateError::PathContainsMap(..) => {
                return None;
            },
            TemplateError::PathContainsUnknownType(..) => {
                return None;
            },
            TemplateError::PathContainsUnknownErrorType(..) => {
                return None;
            },
            TemplateError::FunctionNotFound(..) => {
                return None;
            },
            TemplateError::FunctionError(..) => {
                return None;
            },
            TemplateError::SerializationError => {
                return None;
            },
            TemplateError::UnableConvertBoolToString => {
                return None;
            },
            TemplateError::UnableConvertOptionToString => {
                return None;
            },
            TemplateError::UnableConvertFloatToString => {
                return None;
            },
            TemplateError::VecToUtf8ConvertationError(ref error) => {
                return Some(error);
            },
        }
    }
}