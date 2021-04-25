use crate::function_executor::*;

/// Function: `float:to_string`
/// 
/// Input: `float`
///
/// Returns `string`
pub struct ToString;

impl FunctionExecutor for ToString {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("float:to_string")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_float()?;
        let value = format!("{}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}