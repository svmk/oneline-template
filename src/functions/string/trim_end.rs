use crate::function_executor::*;

/// Function: `string:trim_end`
/// 
/// Input: `String`
///
/// Returns `String`
pub struct TrimEnd;

impl FunctionExecutor for TrimEnd {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("string:trim_end")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_string()?;
        let value = value.trim_end();
        let value = value.to_string();
        let value = Value::String(value);
        return Ok(value);
    }
}