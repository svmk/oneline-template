use crate::template::syntax::token::Token;
use crate::template::syntax::template_token::TemplateToken;
use crate::template::syntax::syntax_parse_error::SyntaxParseError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Syntax {
    tokens: Vec<Token>,
}

impl Syntax {
    pub fn iter_tokens(&self) -> impl Iterator<Item=&Token> {
        return self.tokens.iter();
    }
}


impl FromStr for Syntax {
    type Err = SyntaxParseError;

    fn from_str(mut input: &str) -> Result<Self, Self::Err> {
        let mut result = Syntax {
            tokens: Vec::new(),
        };
        loop {
            let first_template_token_index = TemplateToken::find_start(input);
            match first_template_token_index {
                Some(first_template_token_index) => {
                    let text_token = &input[0..first_template_token_index];
                    if !text_token.is_empty() {
                        result.tokens.push(Token::Text(text_token.to_string()));
                    }
                    let (template, input_remainder) = TemplateToken::parse(&input[first_template_token_index..])?;
                    input = input_remainder;
                    result.tokens.push(Token::Template(template));
                },
                None => {
                    if !input.is_empty() {
                        result.tokens.push(Token::Text(input.to_string()));
                    }
                    break;
                },
            }
        }
        if result.tokens.is_empty() {
            return Err(SyntaxParseError::TemplateIsEmpty);
        }
        return Ok(result);
    }
}