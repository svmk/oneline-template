use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use crate::template::syntax::field_read_token::FieldReadToken;
use crate::template::syntax::field_path::FieldPath;
use crate::template::syntax::function_call_token::FunctionCallToken;
use crate::template::syntax::expected_token_error::ExpectedTokenError;
use crate::template::syntax::unexpected_input_error::UnexpectedInputError;

#[derive(Debug)]
pub struct TemplateToken(FieldReadToken);

impl TemplateToken {
    const FIRST_FORMATTER_TOKEN: char = '{';
    const LAST_FORMATTER_TOKEN: char = '}';
    const SELF_TOKEN: &'static str = "self";
    const FUNC_CALL_TOKEN: &'static str = "|";

    pub fn get_field_read_token(&self) -> &FieldReadToken {
        return &self.0;
    }

    pub fn find_start(input: &str) -> Option<usize> {
        return input.find(Self::FIRST_FORMATTER_TOKEN);
    }
    pub fn parse(mut input: &str) -> Result<(Self, &str), SyntaxParseError> {
        if !input.starts_with(Self::FIRST_FORMATTER_TOKEN) {
            return Err(ExpectedTokenError::new(Self::FIRST_FORMATTER_TOKEN, input).into());
        }
        input = &input[1..];
        input = input.trim_start();
        if input.starts_with(Self::SELF_TOKEN) {
            input = &input[Self::SELF_TOKEN.len()..];
            return Self::parse_field_read(input);
        }
        return Err(UnexpectedInputError::new(input).into());
    }

    fn parse_field_read(input: &str) -> Result<(Self, &str), SyntaxParseError> {
        let (path, input) = FieldPath::parse(input)?;
        let input = input.trim_start();
        let mut input = input;
        let mut function_calls = Vec::new();
        while input.starts_with(Self::FUNC_CALL_TOKEN) {
            input = &input[Self::FUNC_CALL_TOKEN.len()..];
            let (function_call, input_remainder) = FunctionCallToken::parse(input)?;
            input = input_remainder;
            function_calls.push(function_call);
        }
        input = input.trim_start();
        if !input.starts_with(Self::LAST_FORMATTER_TOKEN) {
            return Err(ExpectedTokenError::new(Self::LAST_FORMATTER_TOKEN, input).into());
        }
        input = &input[Self::LAST_FORMATTER_TOKEN.len_utf8()..];
        let result = FieldReadToken::new(path, function_calls);
        let result = TemplateToken(result);
        return Ok((result, input));
    }
}