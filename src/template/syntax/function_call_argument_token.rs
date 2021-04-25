use std::str::FromStr;
use crate::template::syntax::unexpected_input_error::UnexpectedInputError;
use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use crate::template::syntax::invalid_argument_template::InvalidArgumentTemplate;

#[derive(Debug)]
pub enum FunctionCallArgumentToken {
    String(String),
    Bool(bool),
    UInt(u128),
    Int(i128),
}

impl FunctionCallArgumentToken {
    const STRING_TOKEN: char = '\'';
    const ESCAPE_TOKEN: char = '\\';
    const BOOL_TRUE_TOKEN: &'static str = "true";
    const BOOL_FALSE_TOKEN: &'static str = "false";
    const UINT_TOKEN: &'static str = "uint";
    const INT_TOKEN: &'static str = "int";
    pub fn parse(input: &str) -> Result<(FunctionCallArgumentToken, &str), SyntaxParseError> {
        let original_input = input;
        if input.starts_with(Self::STRING_TOKEN) {
            return Self::parse_string(input);
        } else if input.starts_with(Self::BOOL_TRUE_TOKEN) {
            let input = &input[Self::BOOL_TRUE_TOKEN.len()..];
            let result = FunctionCallArgumentToken::Bool(true);
            return Ok((result, input));
        } else if input.starts_with(Self::BOOL_FALSE_TOKEN) {
            let input = &input[Self::BOOL_FALSE_TOKEN.len()..];
            let result = FunctionCallArgumentToken::Bool(false);
            return Ok((result, input));
        }
        let mut number = String::new();
        for c in input.chars() {
            let mut is_allowed = false;
            if c == '-' {
                is_allowed = true;
            }
            if c.is_numeric() {
                is_allowed = true;
            }
            if is_allowed {
                number.push(c);
            } else {
                break;
            }
        }
        let input = &input[number.len()..];
        if input.starts_with(Self::UINT_TOKEN) {
            let input = &input[Self::UINT_TOKEN.len()..];
            let number = u128::from_str(&number)?;
            let result = FunctionCallArgumentToken::UInt(number);
            return Ok((result, input));
        } else if input.starts_with(Self::INT_TOKEN) {
            let input = &input[Self::INT_TOKEN.len()..];
            let number = i128::from_str(&number)?;
            let result = FunctionCallArgumentToken::Int(number);
            return Ok((result, input));
        }
        let text_length = original_input.len() - input.len();
        let invalid_argument_template = &original_input[0..text_length];
        return Err(InvalidArgumentTemplate::new(invalid_argument_template, input).into());
    }

    fn parse_string(input: &str) -> Result<(FunctionCallArgumentToken, &str), SyntaxParseError> {
        let mut result = String::new();
        let mut input = &input[Self::STRING_TOKEN.len_utf8()..];
        let mut is_escape_char = false;
        let mut result_unescaped_length = 0;
        for c in input.chars() {
            if c == Self::STRING_TOKEN {
                if is_escape_char {
                    let _ = result.pop();
                    result.push(c);
                    is_escape_char = false;
                } else {
                    input = &input[result_unescaped_length..];
                    if !input.starts_with(Self::STRING_TOKEN) {
                        break;
                    }
                    input = &input[Self::STRING_TOKEN.len_utf8()..];
                    let result = FunctionCallArgumentToken::String(result);
                    return Ok((result, input));
                }
            } else if c == Self::ESCAPE_TOKEN {
                result.push(c);
                is_escape_char = true;
            } else {
                result.push(c);
                is_escape_char = false;
            }
            result_unescaped_length += c.len_utf8();
        }
        return Err(UnexpectedInputError::new(input).into());
    }
}