use crate::function_executor::*;

/// Function: `int:abs`
/// 
/// Input: `int`
///
/// Returns `uint`
pub struct Abs;

impl FunctionExecutor for Abs {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("int:abs")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_int()?;
        let value = value.abs() as u128;
        let value = Value::UInt(value);
        return Ok(value);
    }
}