use crate::function_executor::*;

/// Function: `uint:hex_fmt`
/// 
/// Input: `uint`
///
/// * first argument: uint number of leading zeroes
///
/// Returns `string`
pub struct HexFmt;

impl FunctionExecutor for HexFmt {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("uint:hex_fmt")
            .with_argument(FunctionArgument::uint())
    }

    fn call(&self, value: Value, arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = value.as_uint()?;
        let value = format!("{:x}", value);
        let leading_zeroes = arguments[0].as_uint()?;
        let leading_zeroes = *leading_zeroes as usize;
        let leading_zeroes = leading_zeroes.saturating_sub(value.len());
        let mut result = "0".repeat(leading_zeroes);
        result += value.as_str();
        let value = Value::String(result);
        return Ok(value);
    }
}