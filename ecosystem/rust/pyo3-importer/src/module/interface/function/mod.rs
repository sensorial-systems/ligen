
use ligen_generator::file_generator::FileSection;
use ligen_ir::{Function, Method, Identifier};

use crate::{prelude::*, type_::TypeGenerator, identifier::IdentifierGenerator};

#[derive(Default)]
pub struct FunctionGenerator {
    type_generator: TypeGenerator,
    identifier_generator: IdentifierGenerator
}

impl FunctionGenerator {
    pub fn generate_function(&self, body: &mut FileSection, function: &Function) -> Result<()> {
        body.write(format!("    pub fn {}(", function.identifier));
        for (index, parameter) in function.inputs.iter().enumerate() {
            let type_ = self.type_generator.translate(&parameter.type_);
            body.write(format!("{}: {}", self.identifier_generator.translate(&parameter.identifier), type_));
            if index < function.inputs.len() - 1 {
                body.write(", ");
            }
        }
        body.write(") ");
        if let Some(output) = &function.output {
            let type_ = self.type_generator.translate(output);
            body.write(format!("-> {} ", type_));
        }
        body.writeln("{");
        if !function.inputs.is_empty() {
            body.writeln("        use pyo3::IntoPy;");    
        }
        body.writeln("        pyo3::Python::with_gil(|py| {");
        body.write("            let args = pyo3::types::PyTuple::new(py, &[");
        for parameter in &function.inputs {
            body.write(format!("{}.into_py(py), ", self.identifier_generator.translate(&parameter.identifier)));
        }
        body.writeln("] as &[pyo3::PyObject]);");
        if function.output.is_some() {
            body.writeln("            let result = PYO3_MODULE.call1(py, args).expect(\"Failed to call method\");");
            body.writeln("            result.extract(py).expect(\"Failed to extract result\")");
        } else {
            body.writeln("            PYO3_MODULE.call1(py, args).expect(\"Failed to call method\");");
        }
        body.writeln("        })");
        body.writeln("    }\n");
        Ok(())
    }

    pub fn generate_method(&self, body: &mut FileSection, method: &Method) -> Result<()> {
        body.write(format!("    pub fn {}(&self", method.identifier));
        for parameter in &method.inputs {
            if parameter.identifier != Identifier::self_() {
                let type_ = self.type_generator.translate(&parameter.type_);
                body.write(format!(", {}: {}", self.identifier_generator.translate(&parameter.identifier), type_));
            }
        }
        body.write(") ");
        if let Some(output) = &method.output {
            let type_ = self.type_generator.translate(output);
            body.write(format!("-> {} ", type_));
        }
        body.writeln("{");
        if method.inputs.len() > 1 {
            body.writeln("        use pyo3::IntoPy;");    
        }

        body.writeln("        pyo3::Python::with_gil(|py| {");
        body.write("            let args = pyo3::types::PyTuple::new(py, &[");
        for parameter in &method.inputs {
            if parameter.identifier != Identifier::self_() {
                body.write(format!("{}.into_py(py), ", self.identifier_generator.translate(&parameter.identifier)));
            }
        }
        body.writeln("] as &[pyo3::PyObject]);");
        if method.output.is_some() {
            body.writeln(format!("            let result = self.object.call_method1(py, \"{}\", args).expect(\"Failed to call method\");", method.identifier));
            body.writeln("            result.extract(py).expect(\"Failed to extract result\")");
        } else {
            body.writeln(format!("            self.object.call_method1(py, \"{}\", args).expect(\"Failed to call method\");", method.identifier));
        }
        body.writeln("        })");
        body.writeln("    }\n");
        Ok(())
    }
}