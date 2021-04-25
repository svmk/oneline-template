use crate::function_executor::*;

/// Function: `bool:unwrap_or`
/// 
/// Input: `Option<bool>`
///
/// * first argument: default bool
///
/// Returns `bool`
pub struct UnwrapOr;

impl FunctionExecutor for UnwrapOr {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("bool:unwrap_or")
            .with_argument(FunctionArgument::bool())
    }

    fn call(&self, value: Value, arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.into_option()?;
        let value = match value {
            Some(value) => {
                value.into_bool()?
            },
            None => {
                arguments[0].as_bool()?.clone()
            },
        };
        let value = Value::Bool(value);
        return Ok(value);
    }
}