use crate::prelude::*;
use ligen_rust_generator::RustTypeGenerator;

#[derive(Default)]
pub struct RustTypeDefinitionGenerator {
    type_generator: RustTypeGenerator
}

impl Generator<&TypeDefinition, String> for RustTypeDefinitionGenerator {
    fn generate(&self, type_def: &TypeDefinition, config: &Config) -> Result<String> {
        let identifier = &type_def.identifier;
        let mut sections = Vec::new();

        for doc in type_def.attributes.get_documentation() {
            for line in doc.lines() {
                sections.push(format!("/// {}", line));
            }
        }

        match &type_def.definition {
            KindDefinition::Structure(structure) => {
                sections.push("#[derive(Debug, Serialize, Deserialize, Clone)]".to_string());
                sections.push(format!("pub struct {} {{", identifier));
                for field in &structure.fields {
                    for doc in field.attributes.get_documentation() {
                        for line in doc.lines() {
                            sections.push(format!("    /// {}", line));
                        }
                    }
                    let field_name = field.identifier.as_ref().map(|i| i.to_string()).unwrap_or_default();
                    let field_type = self.generate_type(&field.type_, config)?;
                    if field_name == "type" {
                        sections.push("    #[serde(rename = \"type\")]".to_string());
                        sections.push(format!("    pub type_: {},", field_type));
                    } else if field_name == "final" {
                        sections.push(format!("    pub r#final: {},", field_type));
                    } else {
                        sections.push(format!("    pub {}: {},", field_name, field_type));
                    }
                }
                sections.push("}".to_string());
            }
            KindDefinition::Enumeration(enumeration) => {
                sections.push("#[derive(Debug, Serialize, Deserialize, Clone)]".to_string());
                sections.push(format!("pub enum {} {{", identifier));
                for variant in &enumeration.variants {
                    for doc in variant.attributes.get_documentation() {
                        for line in doc.lines() {
                            sections.push(format!("    /// {}", line));
                        }
                    }
                    sections.push(format!("    {},", variant.identifier));
                }
                sections.push("}".to_string());
            }
            KindDefinition::TypeAlias(alias) => {
                let type_ = self.generate_type(&alias.type_, config)?;
                sections.push(format!("pub type {} = {};", identifier, type_));
            }
        }
        
        Ok(sections.join("\n"))
    }
}

impl RustTypeDefinitionGenerator {
    fn generate_type(&self, type_: &Type, config: &Config) -> Result<String> {
        let syn_type = self.type_generator.generate(type_, config)?;
        let type_str = quote!(#syn_type).to_string().replace(" ", "");
        Ok(type_str)
    }
}
