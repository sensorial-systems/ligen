use ligen_ir::Module;
use crate::TypeDescriptor;

impl TypeDescriptor for Module {
    fn name() -> String {
        "Module".to_string()
    }

    fn description() -> String {
        "A structure defining a module".to_string()
    }

    fn input_description() -> String {
        "A module definition".to_string()
    }
}
