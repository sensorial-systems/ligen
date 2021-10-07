use super::*;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

impl FunctionProcessor {
    /// Generate function name.
    pub fn generate_function_name(&self, _visitor: &FunctionVisitor) -> String {
        Default::default()
    }

    /// Generate function output.
    pub fn generate_function_output(&self, _output: &Option<ir::Type>) -> String {
        Default::default()
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, file_set: &mut FileSet, function: &Self::Visitor) {
        let file = file_set.entry(&path(function.parent_module()));
        if let FunctionParent::Implementation(implementation) = &function.parent {
            let ffi_name = format!("{}_{}", implementation.parent.definition.identifier(), function.current.identifier);
            // FIXME: Hardcoded DllImport.
            file.writeln(format!("\t\t[DllImport(\"openlimits_csharp\", EntryPoint = \"{}\", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]", ffi_name));
            let return_type = function
                .current
                .output
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or("void".into());
            file.write(format!("\t\tunsafe public static extern {} {}(", return_type, PascalCase::from(SnakeCase::try_from(function.current.identifier.name.as_str()).expect("Failed to transform function name from snake_case to PascalCase."))));
        }
    }

    fn post_process(&self, file_set: &mut FileSet, function: &Self::Visitor) {
        if let FunctionParent::Implementation(_) = &function.parent {
            let file = file_set.entry(&path(function.parent_module()));
            file.writeln(");");
        }
    }
}
