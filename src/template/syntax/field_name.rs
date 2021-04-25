
use crate::template::syntax::syntax_parse_error::SyntaxParseError;
#[derive(Debug)]
pub struct FieldName(String);

impl FieldName {
    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
    
    pub fn parse(input: &str) -> Result<(Self, &str), SyntaxParseError> {
        if input.starts_with(char::is_numeric) {
            return Err(SyntaxParseError::WrongFieldName);
        }
        let mut is_first_char = true;
        let mut field = String::new();
        for c in input.chars() {
            let mut is_allowed = false;
            if !is_first_char && c.is_numeric() {
                is_allowed = true;
            }
            if c.is_alphabetic() {
                is_allowed = true;
            }
            if c == '_' {
                is_allowed = true;
            }
            is_first_char = false;
            if is_allowed {
                field.push(c);
            } else {
                break;
            }
        }
        if field.is_empty() {
            return Err(SyntaxParseError::WrongFieldName);
        }
        let input = &input[field.len()..];
        let field = field.to_string();
        let field = FieldName(field);
        return Ok((field, input));
    }
}