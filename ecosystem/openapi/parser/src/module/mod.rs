use crate::prelude::*;
use ligen_idl::{
    Attribute, Attributes, Field, Function, Identifier, KindDefinition, Module, Named,
    Parameter, PathSegment, Structure, Synchrony, TypeAlias, TypeDefinition, Visibility,
};
use openapiv3::{
    OpenAPI, Operation, ParameterSchemaOrContent, PathItem, ReferenceOr, Responses, Schema,
    SchemaKind, StatusCode, Type as OpenAPIType,
};

#[derive(Default)]
pub struct OpenAPIModuleParser {}

impl OpenAPIModuleParser {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Transformer<&OpenAPI, Module> for OpenAPIModuleParser {
    fn transform(&self, input: &OpenAPI, _config: &Config) -> Result<Module> {
        let mut module = Module::default();
        module.identifier = Identifier::from("root");

        // Map paths to functions
        for (path, item) in input.paths.iter() {
            if let Some(item) = item.as_item() {
                self.parse_path_item(&mut module, path, item)?;
            }
        }

        // Map components/schemas to types
        if let Some(components) = &input.components {
            for (name, schema) in components.schemas.iter() {
                if let Some(schema) = schema.as_item() {
                    let type_definition =
                        self.parse_schema_to_type_definition(&mut module, name, schema)?;
                    module.types.push(type_definition);
                }
            }
        }

        Ok(module)
    }
}

impl OpenAPIModuleParser {
    fn parse_path_item(&self, module: &mut Module, path: &str, item: &PathItem) -> Result<()> {
        let methods = [
            (item.get.as_ref(), "get"),
            (item.post.as_ref(), "post"),
            (item.put.as_ref(), "put"),
            (item.delete.as_ref(), "delete"),
            (item.options.as_ref(), "options"),
            (item.head.as_ref(), "head"),
            (item.patch.as_ref(), "patch"),
            (item.trace.as_ref(), "trace"),
        ];

        for (operation, method) in methods.iter() {
            if let Some(operation) = operation {
                let mut function = self.parse_operation(module, path, method, operation)?;
                for param in &item.parameters {
                    let parameter = match param {
                        ReferenceOr::Reference { reference } => {
                            let name = reference.split('/').last().unwrap_or("Unknown");
                            Ok(Parameter::new(Identifier::from(name), idl::Type::string()))
                        }
                        ReferenceOr::Item(param) => self.parse_parameter(module, param),
                    }?;
                    if !function
                        .inputs
                        .iter()
                        .any(|i| i.identifier == parameter.identifier)
                    {
                        function.inputs.push(parameter);
                    }
                }
                module.functions.push(function);
            }
        }
        Ok(())
    }

    fn parse_operation(
        &self,
        module: &mut Module,
        path: &str,
        method: &str,
        operation: &Operation,
    ) -> Result<Function<()>> {
        let identifier = operation
            .operation_id
            .clone()
            .map(Identifier::from)
            .unwrap_or_else(|| {
                let id = format!(
                    "{}_{}",
                    method,
                    path.replace("/", "_")
                        .replace("{", "_")
                        .replace("}", "_")
                        .replace("__", "_")
                        .trim_matches('_')
                );
                Identifier::from(id)
            })
            .to_snake_case();

        let mut attributes = Attributes::default();
        self.parse_docs(&mut attributes, operation.summary.as_ref(), operation.description.as_ref());

        let mut inputs = Vec::new();
        for param in &operation.parameters {
            if let Some(param) = param.as_item() {
                let parameter = self.parse_parameter(module, param)?;
                inputs.push(parameter);
            }
        }

        let mut path = path.to_string();
        let mut start = 0;
        while let Some(open) = path[start..].find('{') {
            let open = start + open;
            if let Some(close) = path[open..].find('}') {
                let close = open + close;
                let name = &path[open + 1..close];
                let identifier = Identifier::from(name).to_snake_case();
                let identifier_str = identifier.to_string();
                path.replace_range(open + 1..close, &identifier_str);
                if !inputs.iter().any(|i| i.identifier == identifier) {
                    inputs.push(idl::Parameter::new(identifier, idl::Type::string()));
                }
                start = open + 1 + identifier_str.len() + 1;
            } else {
                break;
            }
        }

        let openapi_group = idl::Group::new(
            "openapi",
            vec![
                idl::Attribute::from(Named::new("path", path)),
                idl::Attribute::from(Named::new("method", method)),
            ],
        );
        attributes.push(Attribute::Group(openapi_group));

        if let Some(request_body) = &operation.request_body {
            if let Some(request_body) = request_body.as_item() {
                if let Some(content) = request_body.content.get("application/json") {
                    if let Some(schema) = content.schema.as_ref() {
                        let name = format!("{}Request", identifier.to_pascal_case());
                        let type_ = self.parse_schema_reference(module, schema, Some(name))?;
                        inputs.push(idl::Parameter::new("body", type_));
                    }
                }
            }
        }

        let output = self.parse_responses(module, &identifier, &operation.responses)?;

        Ok(Function {
            attributes,
            visibility: Visibility::Public,
            synchrony: Synchrony::Synchronous,
            identifier,
            inputs,
            output,
            body: (),
        })
    }

    fn parse_parameter(
        &self,
        module: &mut Module,
        param: &openapiv3::Parameter,
    ) -> Result<idl::Parameter> {
        let (name, schema, description) = match param {
            openapiv3::Parameter::Query { parameter_data, .. } => {
                (&parameter_data.name, &parameter_data.format, &parameter_data.description)
            }
            openapiv3::Parameter::Header { parameter_data, .. } => {
                (&parameter_data.name, &parameter_data.format, &parameter_data.description)
            }
            openapiv3::Parameter::Path { parameter_data, .. } => {
                (&parameter_data.name, &parameter_data.format, &parameter_data.description)
            }
            openapiv3::Parameter::Cookie { parameter_data, .. } => {
                (&parameter_data.name, &parameter_data.format, &parameter_data.description)
            }
        };

        let type_ = match schema {
            ParameterSchemaOrContent::Schema(schema) => {
                let name = Identifier::from(name.as_str()).to_pascal_case().to_string();
                self.parse_schema_reference(module, schema, Some(name))?
            }
            _ => idl::Type::opaque(),
        };

        let mut parameter = idl::Parameter::new(
            Identifier::from(name.as_str()).to_snake_case(),
            type_,
        );
        self.parse_docs(&mut parameter.attributes, None, description.as_ref());

        Ok(parameter)
    }

    fn parse_responses(
        &self,
        module: &mut Module,
        identifier: &Identifier,
        responses: &Responses,
    ) -> Result<Option<idl::Type>> {
        let response = responses
            .responses
            .get(&StatusCode::Code(200))
            .or_else(|| responses.responses.get(&StatusCode::Code(201)))
            .or_else(|| responses.default.as_ref());

        if let Some(response) = response {
            if let Some(response) = response.as_item() {
                if let Some(content) = response.content.get("application/json") {
                    if let Some(schema) = content.schema.as_ref() {
                        let name = format!("{}Response", identifier.to_pascal_case());
                        return Ok(Some(self.parse_schema_reference(module, schema, Some(name))?));
                    }
                }
            }
        }
        Ok(None)
    }

    fn parse_schema_reference(
        &self,
        module: &mut Module,
        schema: &ReferenceOr<Schema>,
        name: Option<String>,
    ) -> Result<idl::Type> {
        match schema {
            ReferenceOr::Reference { reference } => {
                let name = reference.split('/').last().unwrap_or("Unknown");
                let identifier = Identifier::from(name).to_pascal_case();
                Ok(idl::Type::from(identifier))
            }
            ReferenceOr::Item(schema) => self.parse_schema_type(module, schema, name),
        }
    }

    fn parse_schema_type(
        &self,
        module: &mut Module,
        schema: &Schema,
        name: Option<String>,
    ) -> Result<idl::Type> {
        match &schema.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                OpenAPIType::String(_) => Ok(idl::Type::string()),
                OpenAPIType::Number(_) => Ok(idl::Type::f64()),
                OpenAPIType::Integer(_) => Ok(idl::Type::i32()),
                OpenAPIType::Boolean(_) => Ok(idl::Type::boolean()),
                OpenAPIType::Array(array) => {
                    let name = name.map(|name| format!("{}Item", name));
                    let items = match &array.items {
                        Some(ReferenceOr::Reference { reference }) => self.parse_schema_reference(
                            module,
                            &ReferenceOr::Reference {
                                reference: reference.clone(),
                            },
                            None,
                        )?,
                        Some(ReferenceOr::Item(item)) => self.parse_schema_type(module, item, name)?,
                        None => idl::Type::opaque(),
                    };
                    Ok(idl::Type::vector(items))
                }
                OpenAPIType::Object(object) => {
                    if let Some(additional_properties) = &object.additional_properties {
                        let type_ = match additional_properties {
                            openapiv3::AdditionalProperties::Any(any) => {
                                if *any {
                                    idl::Type::opaque()
                                } else {
                                    idl::Type::opaque() // Should not happen in valid OpenAPI
                                }
                            }
                            openapiv3::AdditionalProperties::Schema(schema) => {
                                self.parse_schema_reference(module, schema, name)?
                            }
                        };
                        Ok(idl::Type::from(PathSegment::new(
                            Identifier::dictionary(),
                            type_,
                        )))
                    } else if let Some(name) = name {
                        let type_definition =
                            self.parse_schema_to_type_definition(module, &name, schema)?;
                        let id = type_definition.identifier.clone();
                        if !module.types.iter().any(|t| t.identifier == id) {
                            module.types.push(type_definition);
                        }
                        Ok(idl::Type::from(id))
                    } else {
                        Ok(idl::Type::opaque())
                    }
                }
            },
            SchemaKind::AllOf { all_of }
            | SchemaKind::OneOf { one_of: all_of }
            | SchemaKind::AnyOf { any_of: all_of } => {
                let mut structure = Structure::default();
                if let Some(name) = name {
                    let identifier = Identifier::from(name.as_str()).to_pascal_case();
                    self.collect_properties(module, &identifier, all_of, &mut structure)?;
                    let mut type_definition = TypeDefinition {
                        attributes: Default::default(),
                        visibility: Visibility::Public,
                        identifier: identifier.clone(),
                        generics: Default::default(),
                        interfaces: Default::default(),
                        definition: KindDefinition::Structure(structure),
                    };
                    self.parse_docs(&mut type_definition.attributes, schema.schema_data.title.as_ref(), schema.schema_data.description.as_ref());
                    if !module.types.iter().any(|t| t.identifier == identifier) {
                        module.types.push(type_definition);
                    }
                    Ok(idl::Type::from(identifier))
                } else {
                    Ok(idl::Type::opaque())
                }
            }
            _ => Ok(idl::Type::opaque()),
        }
    }

    fn parse_schema_to_type_definition(
        &self,
        module: &mut Module,
        name: &str,
        schema: &Schema,
    ) -> Result<TypeDefinition> {
        let identifier = Identifier::from(name).to_pascal_case();
        let definition = match &schema.schema_kind {
            SchemaKind::Type(OpenAPIType::Object(obj)) => {
                let mut structure = Structure::default();
                for (prop_name, prop_schema) in obj.properties.iter() {
                    let (prop_schema_concrete, prop_desc) = match prop_schema {
                        ReferenceOr::Reference { reference } => (ReferenceOr::Reference { reference: reference.clone() }, None),
                        ReferenceOr::Item(item) => (ReferenceOr::Item(*item.clone()), item.schema_data.description.clone()),
                    };
                    let name = format!(
                        "{}{}",
                        identifier,
                        Identifier::from(prop_name.as_str()).to_pascal_case()
                    );
                    let type_ = self.parse_schema_reference(module, &prop_schema_concrete, Some(name))?;
                    let mut field = Field {
                        attributes: Default::default(),
                        identifier: Some(Identifier::from(prop_name.as_str()).to_snake_case()),
                        type_,
                        visibility: Visibility::Public,
                    };
                    self.parse_docs(&mut field.attributes, None, prop_desc.as_ref());
                    structure.fields.push(field);
                }
                KindDefinition::Structure(structure)
            }
            SchemaKind::AllOf { all_of }
            | SchemaKind::OneOf { one_of: all_of }
            | SchemaKind::AnyOf { any_of: all_of } => {
                let mut structure = Structure::default();
                self.collect_properties(module, &identifier, all_of, &mut structure)?;
                KindDefinition::Structure(structure)
            }
            _ => KindDefinition::TypeAlias(TypeAlias {
                type_: self.parse_schema_type(module, schema, None)?,
            }),
        };

        let mut type_definition = TypeDefinition {
            attributes: Default::default(),
            visibility: Visibility::Public,
            identifier,
            generics: Default::default(),
            interfaces: Default::default(),
            definition,
        };
        self.parse_docs(&mut type_definition.attributes, schema.schema_data.title.as_ref(), schema.schema_data.description.as_ref());
        Ok(type_definition)
    }

    fn collect_properties(
        &self,
        module: &mut Module,
        identifier: &Identifier,
        all_of: &[ReferenceOr<Schema>],
        structure: &mut Structure,
    ) -> Result<()> {
        for schema in all_of {
            let schema = match schema {
                ReferenceOr::Reference { reference } => {
                    let name = reference.split('/').last().unwrap_or("Unknown");
                    let id = Identifier::from(name).to_pascal_case();
                    if let Some(ty) = module.types.iter().find(|t| t.identifier == id) {
                        if let KindDefinition::Structure(s) = &ty.definition {
                            for field in &s.fields {
                                if !structure
                                    .fields
                                    .iter()
                                    .any(|f| f.identifier == field.identifier)
                                {
                                    structure.fields.push(field.clone());
                                }
                            }
                        }
                    }
                    continue;
                }
                ReferenceOr::Item(schema) => schema,
            };

            match &schema.schema_kind {
                SchemaKind::Type(OpenAPIType::Object(obj)) => {
                    for (prop_name, prop_schema) in obj.properties.iter() {
                        let (prop_schema_concrete, prop_desc) = match prop_schema {
                            ReferenceOr::Reference { reference } => (ReferenceOr::Reference { reference: reference.clone() }, None),
                            ReferenceOr::Item(item) => (ReferenceOr::Item(*item.clone()), item.schema_data.description.clone()),
                        };
                        let name = format!(
                            "{}{}",
                            identifier,
                            Identifier::from(prop_name.as_str()).to_pascal_case()
                        );
                        let type_ = self.parse_schema_reference(module, &prop_schema_concrete, Some(name))?;
                        let field_id = Some(Identifier::from(prop_name.as_str()).to_snake_case());
                        if !structure.fields.iter().any(|f| f.identifier == field_id) {
                            let mut field = Field {
                                attributes: Default::default(),
                                identifier: field_id,
                                type_,
                                visibility: Visibility::Public,
                            };
                            self.parse_docs(&mut field.attributes, None, prop_desc.as_ref());
                            structure.fields.push(field);
                        }
                    }
                }
                SchemaKind::AllOf { all_of }
                | SchemaKind::OneOf { one_of: all_of }
                | SchemaKind::AnyOf { any_of: all_of } => {
                    self.collect_properties(module, identifier, all_of, structure)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_docs(
        &self,
        attributes: &mut Attributes,
        summary: Option<&String>,
        description: Option<&String>,
    ) {
        if let Some(summary) = summary {
            attributes.push(Attribute::Named(Named::new("doc", summary.clone())));
        }
        if let Some(description) = description {
            attributes.push(Attribute::Named(Named::new("doc", description.clone())));
        }
    }
}
