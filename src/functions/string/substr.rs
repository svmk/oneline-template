use crate::function_executor::*;

/// Function: `string:substr`
/// 
/// Input: `String`
///
/// * first argument: uint offset
/// * second argument: uint length
///
/// Returns `Option<String>`
pub struct SubStr;

impl FunctionExecutor for SubStr {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("string:substr")
            .with_argument(FunctionArgument::uint())
            .with_argument(FunctionArgument::uint())
    }

    fn call(&self, value: Value, arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_string()?;
        let begin = arguments[0].as_uint()?;
        let begin = *begin as usize;
        let length = arguments[1].as_uint()?;
        let length = *length as usize;
        let end = begin + length;
        let value = value
            .get(begin..end)
            .map(|value| {
                return Value::String(value.to_string());
            })
            .map(Box::new);
        let value = Value::Option(value);
        return Ok(value);
    }
}