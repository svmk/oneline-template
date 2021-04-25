use crate::function_executor::*;

/// Function: `uint:to_string`
/// 
/// Input: `uint`
///
/// Returns `string`
pub struct ToString;

impl FunctionExecutor for ToString {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("uint:to_string")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_uint()?;
        let value = format!("{}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}