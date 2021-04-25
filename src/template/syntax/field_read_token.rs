use crate::template::syntax::field_path::FieldPath;
use crate::template::syntax::function_call_token::FunctionCallToken;

#[derive(Debug)]
pub struct FieldReadToken {
    path: FieldPath,
    function_calls: Vec<FunctionCallToken>,
}

impl FieldReadToken {
    pub fn new(path: FieldPath, function_calls: Vec<FunctionCallToken>) -> FieldReadToken {
        return FieldReadToken {
            path,
            function_calls,
        }
    }

    pub fn get_path(&self) -> &FieldPath {
        return &self.path;
    }

    pub fn get_function_calls(&self) -> &[FunctionCallToken] {
        return &self.function_calls;
    }
}