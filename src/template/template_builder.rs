use crate::template::template_build_error::TemplateBuildError;
use crate::template::template::Template;
use crate::template::argument_types_differ_error::ArgumentTypesDifferError;
use crate::function_executor::FunctionExecutor;
use crate::template::syntax::function_call_token::FunctionCallToken;
use crate::function_executor::FunctionArgumentType;
use crate::template::syntax::syntax::Syntax;
use crate::template::syntax::function_call_argument_token::FunctionCallArgumentToken;
use crate::template::syntax::token::Token;
use crate::template::syntax::template_token::TemplateToken;
use crate::functions;
use std::str::FromStr;
use std::collections::HashMap;


/// Creates template.
pub struct TemplateBuilder {
    functions: HashMap<String, Box<dyn FunctionExecutor>>,
}

impl TemplateBuilder {
    /// Creates template builder WITH default functions.
    pub fn new() -> TemplateBuilder {
        return TemplateBuilder::new_empty()
            .with_function(functions::bool::ToString)
            .with_function(functions::bool::UnwrapOr)
            .with_function(functions::float::ToString)
            .with_function(functions::int::Abs)
            .with_function(functions::int::Hex)
            .with_function(functions::int::HexFmt)
            .with_function(functions::int::ToString)
            .with_function(functions::string::Trim)
            .with_function(functions::string::UnwrapOr)
            .with_function(functions::string::TrimStart)
            .with_function(functions::string::TrimEnd)
            .with_function(functions::string::SubStr)
            .with_function(functions::uint::Hex)
            .with_function(functions::uint::HexFmt)
            .with_function(functions::uint::ToString)
            .with_function(functions::debug::DebugType);
    }

    /// Creates template builder WITHOUT default functions.
    pub fn new_empty() -> TemplateBuilder {
        return TemplateBuilder {
            functions: HashMap::new(),
        }
    }

    /// Adds function executor to template builder.
    pub fn with_function(mut self, function_executor: impl FunctionExecutor + 'static) -> Self {
        let function_executor: Box<dyn FunctionExecutor> = Box::new(function_executor);
        let schema = function_executor.schema();
        let function_name = schema.get_function_name().clone();
        let _ = self.functions.insert(function_name, function_executor);
        return self;
    }

    fn validate_function_call(&self, function_call: &FunctionCallToken) -> Result<(), TemplateBuildError> {
        let function_name = function_call.get_function_name().as_string_ref();
        let function_executor = match self.functions.get(function_name) {
            Some(function_executor) => {function_executor},
            None => {
                return Err(TemplateBuildError::FunctionNotFound(function_name.to_string()));
            },
        };
        let schema = function_executor.schema();
        if function_call.get_arguments().len() != schema.get_arguments().len() {
            return Err(TemplateBuildError::ArgumentsLengthDiffer(function_name.to_string()));
        }
        let iterator = function_call.get_arguments().iter().zip(schema.get_arguments().iter());
        for (argument_index, (actual_argument, expected_argument)) in iterator.enumerate() {
            let actual_argument_type = match actual_argument {
                FunctionCallArgumentToken::String(..) => FunctionArgumentType::String,
                FunctionCallArgumentToken::Bool(..) => FunctionArgumentType::Bool,
                FunctionCallArgumentToken::UInt(..) => FunctionArgumentType::UInt,
                FunctionCallArgumentToken::Int(..) => FunctionArgumentType::Int,
            };
            let expected_argument_type = expected_argument.get_type();
            if &actual_argument_type != expected_argument_type {
                return Err(ArgumentTypesDifferError::new(function_name.to_string(), argument_index).into());
            }
        }
        return Ok(());
    }

    fn validate_template_token(&self, token: &TemplateToken) -> Result<(), TemplateBuildError> {
        let token = token.get_field_read_token();
        for function_call in token.get_function_calls().iter() {
            self.validate_function_call(function_call)?;
        }
        return Ok(());
    }

    fn validate_syntax(&self, syntax: &Syntax) -> Result<(), TemplateBuildError> {
        for token in syntax.iter_tokens() {
            match token {
                Token::Text(..) => {},
                Token::Template(token) => {
                    self.validate_template_token(token)?;
                },
            }
        }
        return Ok(());
    }

    /// Creates template using passed template format.
    pub fn build(self, format: &str) -> Result<Template, TemplateBuildError> {
        let syntax = Syntax::from_str(format)?;
        self.validate_syntax(&syntax)?;
        let template = Template::new(syntax, self.functions);
        return Ok(template);
    }
}