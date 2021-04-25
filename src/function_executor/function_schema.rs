use crate::function_executor::function_argument::FunctionArgument;

/// Contains information about function: function name, argument types.
#[derive(Debug)]
pub struct FunctionSchema {
    function_name: String,
    arguments: Vec<FunctionArgument>,
}

impl FunctionSchema {
    /// Creates function name.
    ///
    /// # Panics
    ///
    /// Panics if `function_name` is empty.
    pub fn new(
        function_name: impl Into<String>,
    ) -> FunctionSchema {
        let function_name = function_name.into();
        if function_name.is_empty() {
            panic!("Passed function name is empty");
        }
        return FunctionSchema {
            function_name,
            arguments: Vec::new(),
        }
    }

    /// Adds function argument.
    pub fn with_argument(mut self, argument: FunctionArgument) -> Self {
        self.arguments.push(argument);
        return self;
    }

    /// Returns function name.
    pub fn get_function_name(&self) -> &String {
        return &self.function_name;
    }


    /// Returns list of arguments.
    pub fn get_arguments(&self) -> &[FunctionArgument] {
        return &self.arguments;
    }
}