use crate::function_executor::*;

/// Function: `int:hex`
/// 
/// Input: `int`
///
/// Returns `string`
pub struct Hex;

impl FunctionExecutor for Hex {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("int:hex")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_int()?;
        let value = format!("{:x}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}