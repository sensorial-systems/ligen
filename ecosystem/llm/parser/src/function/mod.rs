use ligen_idl::Function;
use crate::TypeDescriptor;

impl TypeDescriptor for Function {
    fn name() -> String {
        "Function".to_string()
    }

    fn description() -> String {
        "A structure defining a function".to_string()
    }

    fn input_description() -> String {
        "A function signature".to_string()
    }
}
