use crate::function_executor::Value;
use crate::template::syntax::function_call_token::FunctionCallToken;
use crate::template::syntax::token::Token;
use crate::template::syntax::syntax::Syntax;
use crate::template::syntax::field_read_token::FieldReadToken;
use crate::template::syntax::field::Field;
use crate::function_executor::FunctionExecutor;
use crate::template::TemplateBuilder;
use crate::template::template_build_error::TemplateBuildError;
use std::collections::HashMap;
use std::fmt;
use std::any::type_name;
use unstructured::Document;
use serde::Serialize;
mod convert_document;
use self::convert_document::convert_document;
mod convert_function_argument;
use self::convert_function_argument::convert_function_argument;
mod convert_value_to_string;
use self::convert_value_to_string::convert_value_to_string;
mod template_error;
pub use self::template_error::TemplateError;


/// Templates the passed structure. 
pub struct Template {
    syntax: Syntax,
    functions: HashMap<String, Box<dyn FunctionExecutor>>,
}

impl Template {
    pub (crate) fn new(
        syntax: Syntax,
        functions: HashMap<String, Box<dyn FunctionExecutor>>,
    ) -> Template {
        return Template {
            syntax,
            functions,
        }
    }

    /// Creates template using passed template format.
    pub fn parse(format: &str) -> Result<Template, TemplateBuildError> {
        let builder = TemplateBuilder::new();
        return builder.build(format);
    }

    /// Templates the passed structure.
    pub fn serialize<T>(&self, value: &T) -> Result<String, TemplateError> 
        where 
            T: Serialize,
    {
        let mut result = String::new();
        let document: Document = Document::new(value).map_err(|_| {
            // Anyway there is no information inside unstructured::UnstructuredError.
            return TemplateError::SerializationError;
        })?;
        for token in self.syntax.iter_tokens() {
            match token {
                Token::Text(ref text) => {
                    result += text;
                },
                Token::Template(ref template) => {
                    result += self.read_field(&document, template.get_field_read_token())?.as_str();
                },
            }
        }
        return Ok(result);
    }

    fn read_field(&self, mut document: &Document, template: &FieldReadToken) -> Result<String, TemplateError> {
        for field in template.get_path().get_fields().iter() {
            match field {
                Field::Field(field_name) => {
                    document = &document[field_name.as_str()];
                },
                Field::Index(index) => {
                    document = &document[index];
                },
            }
        }
        let mut value = convert_document(document, template.get_path())?;
        for function_call in template.get_function_calls().iter() {
            value = self.execute_function(value, function_call)?;
        }
        let value = convert_value_to_string(value)?;
        return Ok(value);
    }

    fn execute_function(&self, input: Value, function_call: &FunctionCallToken) -> Result<Value, TemplateError> {
        let function_name = function_call.get_function_name().as_string_ref();
        let function_executor = self
            .functions
            .get(function_name)
            .ok_or_else(|| {
                return TemplateError::FunctionNotFound(function_name.to_string());
            })?;
        let arguments: Vec<_> = function_call
            .get_arguments()
            .iter()
            .map(convert_function_argument)
            .collect();
        let value = function_executor.call(input, &arguments)?;
        return Ok(value);
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut functions = HashMap::new();
        for function_name in self.functions.keys() {
            let _ = functions.insert(function_name, ());
        }
        f.debug_struct(type_name::<Self>())
        .field("syntax", &self.syntax)
        .field("functions", &functions)
        .finish()
    }
}