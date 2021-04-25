use crate::function_executor::*;

/// Function: `string:trim`
/// 
/// Input: `String`
///
/// Returns `String`
pub struct Trim;

impl FunctionExecutor for Trim {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("string:trim")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_string()?;
        let value = value.trim();
        let value = value.to_string();
        let value = Value::String(value);
        return Ok(value);
    }
}