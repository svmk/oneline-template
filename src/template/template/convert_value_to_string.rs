use crate::function_executor::Value;
use crate::template::template_error::TemplateError;

pub fn convert_value_to_string(value: Value) -> Result<String, TemplateError> {
    let value = match value {
        Value::String(value) => {
            value
        },
        Value::Bool(..) => {
            return Err(TemplateError::UnableConvertBoolToString);
        },
        Value::Bytes(value) => {
            let value = String::from_utf8(value)
                .map_err(|error| {
                    return TemplateError::VecToUtf8ConvertationError(error);
                })?;
            value
        },
        Value::Char(value) => {
            value.to_string()
        },
        Value::Option(..) => {
            return Err(TemplateError::UnableConvertOptionToString);
        },
        Value::UInt(value) => {
            value.to_string()
        },
        Value::Int(value) => {
            value.to_string()
        },
        Value::Float(..) => {
            return Err(TemplateError::UnableConvertFloatToString);
        },
    };
    return Ok(value);
}