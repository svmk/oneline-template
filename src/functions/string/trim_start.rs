use crate::function_executor::*;

/// Function: `string:trim_start`
/// 
/// Input: `String`
///
/// Returns `String`
pub struct TrimStart;

impl FunctionExecutor for TrimStart {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("string:trim_start")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_string()?;
        let value = value.trim_start();
        let value = value.to_string();
        let value = Value::String(value);
        return Ok(value);
    }
}