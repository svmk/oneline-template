use crate::function_executor::*;

/// Function: `uint:hex`
/// 
/// Input: `uint`
///
/// Returns `string`
pub struct Hex;

impl FunctionExecutor for Hex {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("uint:hex")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_uint()?;
        let value = format!("{:x}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}