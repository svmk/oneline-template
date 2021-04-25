use crate::function_executor::*;

/// Function: `bool:to_string`
/// 
/// Input: `bool`
///
/// Returns `string`
pub struct ToString;

impl FunctionExecutor for ToString {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("bool:to_string")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_bool()?;
        let value = format!("{}", value);
        let value = Value::String(value);
        return Ok(value);
    }
}