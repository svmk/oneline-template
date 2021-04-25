use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use crate::template::syntax::field::Field;
use std::fmt;

#[derive(Debug)]
pub struct FieldPath {
    path: Vec<Field>,
}

impl FieldPath {
    const DOT: char = '.';
    pub fn get_fields(&self) -> &[Field] {
        return &self.path;
    }
    
    pub fn parse(mut input: &str) -> Result<(FieldPath, &str), SyntaxParseError> {
        let mut result = FieldPath {
            path: Vec::new(),
        };
        while input.starts_with(Self::DOT) {
            input = &input[Self::DOT.len_utf8()..];
            let (field, input_remainder) = Field::parse(input)?;
            result.path.push(field);
            input = input_remainder;
        }
        return Ok((result, input));
    }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_first = true;
        for field in self.path.iter() {
            if !is_first {
                write!(f, ".")?;
            }
            match field {
                &Field::Index(index) => {
                    write!(f, "{}", index)?;
                },
                &Field::Field(ref field) => {
                    write!(f, "{}", field.as_str())?;
                },
            }
            is_first = false;
        }
        return Ok(());
    }
}