use serde::{Serialize, Serializer};
use crate::function_executor::function_error::FunctionError;

/// The value used for templating.
/// 
/// ## Automatic convertation into string
/// 
/// * `string` -> `string`
/// * `char`   -> `string`
/// * `uint`   -> `string` using decimal base.
/// * `int`    -> `string` using decimal base with sign.
/// * `bytes`  -> utf-8 encoded string or error
/// * `bool`   -> error. For convertation boolean type use `bool:to_string`
/// * `option` -> error. For convertation option use `string:unwrap_or`, `uint:unwrap_or` or others.
/// * `float`  -> error. For convertation option use `float:to_string`.
#[derive(Debug)]
pub enum Value {
    /// Value that contains string type.
    String(String),
    /// Value that contains boolean type.
    Bool(bool),
    /// Value that contains array of bytes.
    Bytes(Vec<u8>),
    /// Value that contains char.
    Char(char),
    /// Value that may contain other value.
    Option(Option<Box<Value>>),
    /// Value that contains unsigned 128-bit integer.
    UInt(u128),
    /// Value that contains signed 128-bit integer.
    Int(i128),
    /// Value that contains float value.
    Float(f64),
}

impl Value {
    /// Trying to cast value as string.
    pub fn as_string(&self) -> Result<&String, FunctionError> {
        if let Value::String(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as string", self)));
    }

    /// Trying to cast value as mutable string.
    pub fn as_mut_string(&mut self) -> Result<&mut String, FunctionError> {
        if let Value::String(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as string", self)));
    }

    /// Trying to cast value as string.
    pub fn into_string(self) -> Result<String, FunctionError> {
        if let Value::String(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as string", self)));
    }

    /// Trying to cast value as bool.
    pub fn as_bool(&self) -> Result<&bool, FunctionError> {
        if let Value::Bool(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bool", self)));
    }

    /// Trying to cast value as mutable bool.
    pub fn as_mut_bool(&mut self) -> Result<&mut bool, FunctionError> {
        if let Value::Bool(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bool", self)));
    }

    /// Trying to cast value as bool.
    pub fn into_bool(self) -> Result<bool, FunctionError> {
        if let Value::Bool(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bool", self)));
    }

    /// Trying to cast value as bytes.
    pub fn as_bytes(&self) -> Result<&Vec<u8>, FunctionError> {
        if let Value::Bytes(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bytes", self)));
    }

    /// Trying to cast value as mutable bytes.
    pub fn as_mut_bytes(&mut self) -> Result<&mut Vec<u8>, FunctionError> {
        if let Value::Bytes(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bytes", self)));
    }

    /// Trying to cast value as bytes.
    pub fn into_bytes(self) -> Result<Vec<u8>, FunctionError> {
        if let Value::Bytes(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as bytes", self)));
    }

    /// Trying to cast value as char.
    pub fn as_char(&self) -> Result<&char, FunctionError> {
        if let Value::Char(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as char", self)));
    }

    /// Trying to cast value as mutable char.
    pub fn as_mut_char(&mut self) -> Result<&mut char, FunctionError> {
        if let Value::Char(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as char", self)));
    }

    /// Trying to cast value as option.
    pub fn as_option(&self) -> Result<Option<&Value>, FunctionError> {
        if let Value::Option(ref value) = self {
            let value = value.as_ref();
            let value = value.map(Box::as_ref);
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as option", self)));
    }

    /// Trying to cast value as mutable option.
    pub fn as_mut_option(&mut self) -> Result<Option<&mut Value>, FunctionError> {
        if let Value::Option(ref mut value) = self {
            let value = value.as_mut();
            let value = value.map(Box::as_mut);
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as option", self)));
    }

    /// Trying to cast value as option.
    pub fn into_option(self) -> Result<Option<Value>, FunctionError> {
        if let Value::Option(value) = self {
            let value = value.map(|value| {
                return *value;
            });
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as option", self)));
    }

    /// Trying to cast value as uint.
    pub fn as_uint(&self) -> Result<&u128, FunctionError> {
        if let Value::UInt(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as uint", self)));
    }

    /// Trying to cast value as mutable uint.
    pub fn as_mut_uint(&mut self) -> Result<&mut u128, FunctionError> {
        if let Value::UInt(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as uint", self)));
    }

    /// Trying to cast value as uint.
    pub fn into_uint(self) -> Result<u128, FunctionError> {
        if let Value::UInt(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as uint", self)));
    }

    /// Trying to cast value as int.
    pub fn as_int(&self) -> Result<&i128, FunctionError> {
        if let Value::Int(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as int", self)));
    }

    /// Trying to cast value as mutable int.
    pub fn as_mut_int(&mut self) -> Result<&mut i128, FunctionError> {
        if let Value::Int(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as int", self)));
    }

    /// Trying to cast value as int.
    pub fn into_int(self) -> Result<i128, FunctionError> {
        if let Value::Int(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as int", self)));
    }

    /// Trying to cast value as float.
    pub fn as_float(&self) -> Result<&f64, FunctionError> {
        if let Value::Float(ref value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as float", self)));
    }

    /// Trying to cast value as mutable float.
    pub fn as_mut_float(&mut self) -> Result<&mut f64, FunctionError> {
        if let Value::Float(ref mut value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as float", self)));
    }

    /// Trying to cast value as float.
    pub fn floato_float(self) -> Result<f64, FunctionError> {
        if let Value::Float(value) = self {
            return Ok(value);
        }
        return Err(FunctionError::msg(format!("Trying to cast {:?} as float", self)));
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer 
    {
        match self {
            Value::String(ref value) => {
                return value.serialize(serializer);
            },
            Value::Bool(ref value) => {
                return value.serialize(serializer);
            },
            Value::Bytes(ref value) => {
                return value.serialize(serializer);
            },
            Value::Char(ref value) => {
                return value.serialize(serializer);
            },
            Value::Option(ref value) => {
                return value.serialize(serializer);
            },
            Value::UInt(ref value) => {
                return value.serialize(serializer);
            },
            Value::Int(ref value) => {
                return value.serialize(serializer);
            },
            Value::Float(ref value) => {
                return value.serialize(serializer);
            },
        }
    }
}