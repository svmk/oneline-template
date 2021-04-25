use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use crate::template::syntax::field_name::FieldName;
use std::str::FromStr;

#[derive(Debug)]
pub enum Field {
    Field(FieldName),
    Index(usize),
}

impl Field {
    pub fn parse(input: &str) -> Result<(Self, &str), SyntaxParseError> {
        let first_char = input.chars().nth(0);
        let first_char = match first_char {
            Some(first_char) => first_char,
            None => {
                return Err(SyntaxParseError::WrongFieldName);
            },
        };
        if first_char.is_numeric() {
            let mut index = String::new();
            for c in input.chars() {
                let mut is_allowed = false;
                if c.is_numeric() {
                    is_allowed = true;
                }
                if is_allowed {
                    index.push(c);
                } else {
                    break;
                }
            }
            let input = &input[index.len()..];
            let index = usize::from_str(&index)?;
            let index = Field::Index(index);
            return Ok((index, input));
        }
        let (field_name, input) = FieldName::parse(input)?;
        let field_name = Field::Field(field_name);
        return Ok((field_name, input));
    }
}