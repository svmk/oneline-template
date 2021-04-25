use crate::function_executor::*;

/// Function: `debug:type`
/// 
/// Input: any type
///
/// Returns `string`
pub struct DebugType;

impl FunctionExecutor for DebugType {
    fn schema(&self) -> FunctionSchema {
        FunctionSchema::new("debug:type")
    }

    fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
        let value = inspect_type(value);
        let value = Value::String(value);
        return Ok(value);
    }
}

fn inspect_type(value: Value) -> String {
    match value {
        Value::String(..) => {
            return "String".to_string();
        },
        Value::Bool(..) => {
            return "Bool".to_string();
        },
        Value::Bytes(..) => {
            return "Bytes".to_string();
        },
        Value::Char(..) => {
            return "Char".to_string();
        },
        Value::Option(value) => {
            match value {
                Some(value) => {
                    let value = inspect_type(*value);
                    return format!("Option<{}>", value);
                },
                None => {
                    return "Option<..>".to_string();        
                },
            }
        },
        Value::UInt(..) => {
            return "UInt".to_string();
        },
        Value::Int(..) => {
            return "Int".to_string();
        },
        Value::Float(..) => {
            return "Float".to_string();
        },
    }
}