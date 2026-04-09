use crate::prelude::*;
use ligen_rust_generator::RustTypeGenerator;
use anyhow::Context;

#[derive(Default)]
pub struct RustFunctionGenerator {
    type_generator: RustTypeGenerator
}

impl Generator<&Function<()>, String> for RustFunctionGenerator {
    fn generate(&self, function: &Function<()>, config: &Config) -> Result<String> {
        let openapi = function.attributes.get_subgroup("openapi")
            .context("Missing #[openapi] attribute")?;
        
        let path = openapi.get_named("path")
            .and_then(|l| l.as_string())
            .context("Missing path in #[openapi]")?;
            
        let method = openapi.get_named("method")
            .and_then(|l| l.as_string())
            .context("Missing method in #[openapi]")?;

        let mut docs = function.attributes.get_documentation();
        let parameter_docs: Vec<_> = function.inputs.iter()
            .filter_map(|input| {
                let docs = input.attributes.get_documentation();
                if docs.is_empty() {
                    None
                } else {
                    Some(format!("- `{}`: {}", input.identifier, docs.join(" ")))
                }
            })
            .collect();

        if !parameter_docs.is_empty() {
            docs.push("".to_string());
            docs.push("# Parameters".to_string());
            docs.extend(parameter_docs);
        }

        let identifier = &function.identifier;
        let mut inputs = Vec::new();
        for input in &function.inputs {
            let name = &input.identifier;
            let type_ = self.generate_type(&input.type_, config)?;
            inputs.push(format!("{}: {}", name, type_));
        }
        
        let output = if let Some(output) = &function.output {
            self.generate_type(output, config)?
        } else {
            "()".to_string()
        };

        let mut body = Vec::new();
        let mut rust_path = path.to_string();
        let mut url_args = vec!["self.base_url.clone()".to_string()];
        let mut path_parameters = Vec::new();
        for input in &function.inputs {
            let name = input.identifier.to_string();
            let placeholder = format!("{{{}}}", name);
            if rust_path.contains(&placeholder) {
                rust_path = rust_path.replace(&placeholder, "{}");
                url_args.push(name.clone());
                path_parameters.push(name);
            }
        }
        body.push(format!(r#"let url = format!("{{}}{}", {});"#, rust_path, url_args.join(", ")));
        
        let mut modifications = Vec::new();
        for input in &function.inputs {
            let name = input.identifier.to_string();
            if name == "body" {
                modifications.push("request = request.json(&body);".to_string());
            } else if !path_parameters.contains(&name) {
                modifications.push(format!("request = request.query(&[(\"{}\", &{})]);", name, name));
            }
        }

        if modifications.is_empty() {
            body.push(format!("let request = self.client.{}(url);", method.to_lowercase()));
        } else {
            body.push(format!("let mut request = self.client.{}(url);", method.to_lowercase()));
            body.extend(modifications);
        }
        
        let response_name = if output == "()" { "_response" } else { "response" };
        body.push(format!("let {} = request.send().await?;", response_name));

        if output != "()" {
           body.push(format!("let result: {} = response.json().await?;", output));
           body.push("Ok(result)".to_string());
        } else {
           body.push("Ok(())".to_string());
        }

        let mut sections = Vec::new();
        for doc in docs {
            for line in doc.lines() {
                sections.push(format!("/// {}", line));
            }
        }
        sections.push(format!("pub async fn {}(&self, {}) -> Result<{}, reqwest::Error> {{", identifier, inputs.join(", "), output));
        for line in body {
            sections.push(format!("    {}", line));
        }
        sections.push("}".to_string());

        Ok(sections.join("\n"))
    }
}

impl RustFunctionGenerator {
    fn generate_type(&self, type_: &Type, config: &Config) -> Result<String> {
        let syn_type = self.type_generator.generate(type_, config)?;
        let type_str = quote!(#syn_type).to_string().replace(" ", "");
        Ok(type_str)
    }
}
