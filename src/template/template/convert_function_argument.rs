use crate::template::syntax::function_call_argument_token::FunctionCallArgumentToken;
use crate::function_executor::Value;

pub fn convert_function_argument(argument: &FunctionCallArgumentToken) -> Value {
    match argument {
        &FunctionCallArgumentToken::String(ref value) => {
            Value::String(value.clone())
        },
        &FunctionCallArgumentToken::Bool(value) => {
            Value::Bool(value)
        },
        &FunctionCallArgumentToken::UInt(value) => {
            Value::UInt(value)
        },
        &FunctionCallArgumentToken::Int(value) => {
            Value::Int(value)
        },
    }
}