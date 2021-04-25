use crate::function_executor::value::Value;
use crate::function_executor::function_schema::FunctionSchema;
use crate::function_executor::function_error::FunctionError;


/// Executes function.
pub trait FunctionExecutor {
    /// Returns function schema that contain information about function: function name and argument types.
    fn schema(&self) -> FunctionSchema;
    /// Executes function.
    /// * `input` value that retrieved from field or other function executor.
    /// * `arguments` list of arguments that was declared within template.
    fn call(&self, input: Value, arguments: &[Value]) -> Result<Value, FunctionError>;
}