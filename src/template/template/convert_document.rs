use crate::template::syntax::field_path::FieldPath;
use crate::function_executor::Value;
use crate::template::template_error::TemplateError;
use unstructured::{Document, Unstructured, Number};

pub fn convert_document(document: &Document, path: &FieldPath) -> Result<Value, TemplateError> {
    let value = match document {
        &Unstructured::Unassigned => {
            return Err(TemplateError::PathNotFound(format!("{}", path)));
        },
        &Unstructured::Null => {
            return Err(TemplateError::PathNotFound(format!("{}", path)));
        },
        &Unstructured::Bool(value) => {
            Value::Bool(value)
        },
        &Unstructured::Number(ref value) => {
            match value {
                &Number::U8(value) => {
                    Value::UInt(value as u128)
                },
                &Number::U16(value) => {
                    Value::UInt(value as u128)
                },
                &Number::U32(value) => {
                    Value::UInt(value as u128)
                },
                &Number::U64(value) => {
                    Value::UInt(value as u128)
                },
                &Number::U128(value) => {
                    Value::UInt(value)
                },
                &Number::I8(value) => {
                    Value::Int(value as i128)
                },
                &Number::I16(value) => {
                    Value::Int(value as i128)
                },
                &Number::I32(value) => {
                    Value::Int(value as i128)
                },
                &Number::I64(value) => {
                    Value::Int(value as i128)
                },
                &Number::I128(value) => {
                    Value::Int(value as i128)
                },
                &Number::F32(value) => {
                    Value::Float(value as f64)
                },
                &Number::F64(value) => {
                    Value::Float(value)
                },
            }
        },
        &Unstructured::String(ref value) => {
            Value::String(value.clone())
        },
        &Unstructured::Char(value) => {
            Value::Char(value)
        },
        &Unstructured::Bytes(ref value) => {
            Value::Bytes(value.clone())
        },
        &Unstructured::Seq(ref value) => {
            let mut data: Vec<u8> = Vec::new();
            for item in value.iter() {
                if let Unstructured::Number(Number::U8(item)) = item {
                    data.push(*item);
                } else {
                    return Err(TemplateError::PathContainsSeq(format!("{}", path)));        
                }
            }
            Value::Bytes(data.clone())
        },
        &Unstructured::Map(..) => {
            return Err(TemplateError::PathContainsMap(format!("{}", path)));
        },
        &Unstructured::Option(ref value) => {
            let value = match value {
                Some(value) => {
                    let value = convert_document(value.as_ref(), path)?;
                    let value = Box::new(value);
                    Some(value)
                },
                None => None,
            };
            Value::Option(value)
        },
        &Unstructured::Newtype(..) => {
            return Err(TemplateError::PathContainsUnknownType(format!("{}", path)));
        },
        &Unstructured::Err(..) => {
            return Err(TemplateError::PathContainsUnknownErrorType(format!("{}", path)));
        },
        &Unstructured::Other(..) => {
            return Err(TemplateError::PathContainsUnknownType(format!("{}", path)));
        },
    };
    return Ok(value);
}