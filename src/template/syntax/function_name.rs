use crate::template::syntax::syntax_parse_error::SyntaxParseError;

#[derive(Debug)]
pub struct FunctionName(String);

impl FunctionName {
    pub fn as_string_ref(&self) -> &String {
        return &self.0;
    }
    
    pub fn parse(input: &str) -> Result<(Self, &str), SyntaxParseError> {
        if input.starts_with(char::is_numeric) {
            return Err(SyntaxParseError::WrongFunctionName);
        }
        let mut function = String::new();
        let mut is_first_char = true;
        for c in input.chars() {
            let mut is_allowed = false;
            if c.is_alphabetic() {
                is_allowed = true;
            }
            if !is_first_char && c.is_numeric() {
                is_allowed = true;
            }
            if c == '_' {
                is_allowed = true;
            }
            if c == ':' {
                is_allowed = true;
            }
            is_first_char = false;
            if is_allowed {
                function.push(c);
            } else {
                break;
            }
        }
        if function.is_empty() {
            return Err(SyntaxParseError::WrongFunctionName);
        }
        let input = &input[function.len()..];
        let function = function.to_string();
        let function = FunctionName(function);
        return Ok((function, input));
    }
}