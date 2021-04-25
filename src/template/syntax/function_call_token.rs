use crate::template::syntax::function_call_argument_token::FunctionCallArgumentToken;
use crate::template::syntax::function_name::FunctionName;
use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use crate::template::syntax::expected_token_error::ExpectedTokenError;

#[derive(Debug)]
pub struct FunctionCallToken {
    function_name: FunctionName,
    arguments: Vec<FunctionCallArgumentToken>,
}

impl FunctionCallToken {
    const OPEN_BRAKET_TOKEN: char = '(';
    const CLOSE_BRAKET_TOKEN: char = ')';
    const COMMA_TOKEN: char = ',';

    pub fn get_function_name(&self) -> &FunctionName {
        return &self.function_name;
    }

    pub fn get_arguments(&self) -> &[FunctionCallArgumentToken] {
        return &self.arguments;
    }
    
    pub fn parse(input: &str) -> Result<(FunctionCallToken, &str), SyntaxParseError> {
        let input = input.trim_start();
        let (function_name, input) = FunctionName::parse(input)?;
        let input = input.trim_start();
        if !input.starts_with(Self::OPEN_BRAKET_TOKEN) {
            return Err(ExpectedTokenError::new(Self::OPEN_BRAKET_TOKEN, input).into());
        }
        let mut input = &input[Self::OPEN_BRAKET_TOKEN.len_utf8()..];
        let mut result = FunctionCallToken {
            function_name,
            arguments: Vec::new(),
        };
        loop {
            input = input.trim_start();
            if !input.starts_with(Self::CLOSE_BRAKET_TOKEN) {
                let (argument, input_remainder) = FunctionCallArgumentToken::parse(input)?;
                result.arguments.push(argument);
                input = input_remainder;
            }
            input = input.trim_start();
            if input.starts_with(Self::CLOSE_BRAKET_TOKEN) {
                input = &input[Self::CLOSE_BRAKET_TOKEN.len_utf8()..];
                input = input.trim_start();
                break;
            }
            input = input.trim_start();
            if !input.starts_with(Self::COMMA_TOKEN) {
                return Err(ExpectedTokenError::new(Self::COMMA_TOKEN, input).into());
            }
            input = &input[Self::COMMA_TOKEN.len_utf8()..];
        }
        return Ok((result, input));
    }
}