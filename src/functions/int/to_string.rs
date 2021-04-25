use crate::function_executor::*;

/// Function: `int:to_string`
/// 
/// Input: `int`
///
/// Returns `string`
pub struct ToString;

impl FunctionExecutor for ToString {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("int:to_string")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_int()?;
        let value = format!("{}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}