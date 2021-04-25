use crate::function_executor::function_argument_type::FunctionArgumentType;

/// Information about function argument schema.
#[derive(Debug)]
pub struct FunctionArgument {
    argument_type: FunctionArgumentType,
}

impl FunctionArgument {
    /// Creates function argument schema with string type.
    pub fn string() -> FunctionArgument {
        return FunctionArgument {
            argument_type: FunctionArgumentType::String,
        }
    }

    /// Creates function argument schema with bool type.
    pub fn bool() -> FunctionArgument {
        return FunctionArgument {
            argument_type: FunctionArgumentType::Bool,
        }
    }

    /// Creates function argument schema with uint type.
    pub fn uint() -> FunctionArgument {
        return FunctionArgument {
            argument_type: FunctionArgumentType::UInt,
        }
    }
    
    /// Creates function argument schema with int type.
    pub fn int() -> FunctionArgument {
        return FunctionArgument {
            argument_type: FunctionArgumentType::Int,
        }
    }

    pub (crate) fn get_type(&self) -> &FunctionArgumentType {
        return &self.argument_type;
    }
}