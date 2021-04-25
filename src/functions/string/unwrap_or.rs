use crate::function_executor::*;

/// Function: `string:unwrap_or`
/// 
/// Input: `Option<String>`
///
/// * first argument: default string
///
/// Returns `String`
pub struct UnwrapOr;

impl FunctionExecutor for UnwrapOr {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("string:unwrap_or")
            .with_argument(FunctionArgument::string())
    }

    fn call(&self, value: Value, arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.into_option()?;
        let value = match value {
            Some(value) => {
                value.into_string()?
            },
            None => {
                arguments[0].as_string()?.clone()
            },
        };
        let value = Value::String(value);
        return Ok(value);
    }
}