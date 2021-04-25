/// Error reason when passed argument type is differ from required. 
#[derive(Debug)]
pub struct ArgumentTypesDifferError {
    function_name: String,
    argument_position: usize,
}

impl ArgumentTypesDifferError {
    pub (crate) fn new(
        function_name: String,
        argument_position: usize,
    ) -> ArgumentTypesDifferError {
        return ArgumentTypesDifferError {
            function_name,
            argument_position,
        }
    }
    
    /// Returns function name that was called when error was occupied.
    pub fn get_function_name(&self) -> &String {
        return &self.function_name;
    }
    
    /// Returns argument position. Indexing started from zero.
    pub fn get_argument_position(&self) -> usize {
        return self.argument_position;
    }
}