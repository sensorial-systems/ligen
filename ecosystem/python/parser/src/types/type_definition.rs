use crate::{prelude::*, identifier::IdentifierParser, macro_attributes::attributes::AttributesParser, function::FunctionParser, types::type_::TypeParser};
use ligen::{ir::{TypeDefinition, Visibility, Path, KindDefinition, Structure, Attribute, Field}, parsing::parser::ParserConfig};
use rustpython_parser::ast::{StmtClassDef, Expr, Stmt, StmtAnnAssign, StmtAugAssign, StmtAssign};

#[derive(Default)]
pub struct TypeDefinitionParser {}

impl Parser<WithSource<StmtClassDef>> for TypeDefinitionParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        if config.only_parse_symbols() {
            Ok(TypeDefinition { identifier, ..Default::default() })
        } else {
            let attributes = AttributesParser::default().parse(input.sub(input.ast.decorator_list.clone()), config)?;
            let visibility = Visibility::Public;
            let interfaces = self.parse_interfaces(&input.ast.bases, config)?;
            let definition = self.parse_kind_definition(&input, config)?;
            let generics = Default::default();
            Ok(TypeDefinition { attributes, visibility, identifier, generics, definition, interfaces })
        }
    }
}

impl TypeDefinitionParser {
    fn parse_interfaces(&self, input: &Vec<Expr>, config: &ParserConfig) -> Result<Vec<Path>> {
        let mut interfaces = Vec::new();
        for expr in input {
            if let Some(expr) = expr.as_name_expr() {
                interfaces.push(IdentifierParser::default().parse(expr.id.as_str(), config)?.into());
            }
        }
        Ok(interfaces)
    }

    fn parse_field_from_ann_assign(&self, input: &WithSource<&StmtAnnAssign>, config: &ParserConfig) -> Result<Field> {
        let identifier = input
            .ast
            .target
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let identifier = Some(identifier);
        let type_ = TypeParser::new().parse(&*input.ast.annotation, config)?;
        let visibility = Default::default();
        let attributes = Default::default();
        Ok(Field { identifier, type_, visibility, attributes })
    }

    fn parse_field_from_aug_assign(&self, input: &WithSource<&StmtAugAssign>, config: &ParserConfig) -> Result<Field> {
        let identifier = input
            .ast
            .target
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let identifier = Some(identifier);
        let type_ = Default::default();
        let visibility = Default::default();
        let attributes = Default::default();
        Ok(Field { identifier, type_, visibility, attributes })
    }

    fn parse_fields_from_assign(&self, input: &WithSource<&StmtAssign>, config: &ParserConfig) -> Result<Vec<Field>> {
        let mut fields = Vec::new();
        for target in &input.ast.targets {
            if let Some(identifier) = target.as_name_expr() {
                let identifier = IdentifierParser::new().parse(identifier.id.as_str(), config)?;
                let identifier = Some(identifier);
                let type_ = Default::default();
                let visibility = Default::default();
                let attributes = Default::default();
                let field = Field { identifier, type_, visibility, attributes };
                fields.push(field);
            }
        }
        Ok(fields)
    }

    fn parse_kind_definition(&self, input: &WithSource<StmtClassDef>, config: &ParserConfig) -> Result<KindDefinition> {
        let mut fields = Vec::new();
        let class_variables_as_properties = config
            .get("class_variables_as_properties")
            .and_then(|x| x.as_boolean())
            .cloned()
            .unwrap_or_default();
        for stmt in &input.ast.body {
            match stmt {
                Stmt::AnnAssign(ann_assign) => {
                    if class_variables_as_properties {
                        let field = self.parse_field_from_ann_assign(&input.sub(ann_assign), config)?;
                        fields.push(field);
                    }
                },
                Stmt::AugAssign(aug_assign) => {
                    if class_variables_as_properties {
                        let field = self.parse_field_from_aug_assign(&input.sub(aug_assign), config)?;
                        fields.push(field)
                    }
                },
                Stmt::Assign(assign) => {
                    if class_variables_as_properties {
                        let parsed_fields = self.parse_fields_from_assign(&input.sub(assign), config)?;
                        fields.extend(parsed_fields);
                    }
                },
                Stmt::FunctionDef(function_def) => {
                    let function = FunctionParser::default().parse(input.sub(function_def.clone()), config)?;
                    if function.attributes.contains(&Attribute::Group("property".into(), Default::default())) {
                        let identifier = Some(function.identifier);
                        let type_ = function.output.unwrap_or_default();
                        let field = Field { identifier, type_, ..Default::default() };
                        fields.push(field);
                    }    
                },
                _ => ()
            }
        }
        let structure = Structure { fields };
        Ok(structure.into())
    }
}
