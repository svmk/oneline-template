use std::fmt;
use std::error::Error;

/// Function error.
pub struct FunctionError {
    display: String,
    debug: String,
}

impl FunctionError {
    /// Creates function error.
    pub fn msg<E>(error: E) -> FunctionError 
        where E: fmt::Display + fmt::Debug + 'static,
    {
        let display = format!("{}", error);
        let debug = format!("{:?}", error);
        return FunctionError {
            display,
            debug,
        }
    }
}

impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl fmt::Debug for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl <E>From<E> for FunctionError 
    where 
        E: Error,
        E: 'static,
{
    fn from(error: E) -> Self {
        return Self::msg(error);
    }
}