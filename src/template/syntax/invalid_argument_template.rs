use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidArgumentTemplate {
    template: String,
    input: String,
}

impl InvalidArgumentTemplate {
    pub (crate) fn new(
        template: impl Into<String>,
        input: impl Into<String>,
    ) -> InvalidArgumentTemplate {
        return InvalidArgumentTemplate {
            template: template.into(),
            input: input.into(),
        }
    }
}

impl fmt::Display for InvalidArgumentTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid argument `{}` in input `{}`", self.template, self.input)
    }
}

impl Error for InvalidArgumentTemplate {}