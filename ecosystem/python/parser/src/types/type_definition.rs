use std::collections::HashMap;

use crate::{prelude::*, identifier::IdentifierParser, macro_attributes::attributes::AttributesParser, function::FunctionParser, types::type_::TypeParser, parser::PythonParserConfig};
use ligen::{ir::{Type, TypeDefinition, Path, KindDefinition, Structure, Field}, parser::ParserConfig};
use ligen::ir::Mutability;
use rustpython_parser::ast::{StmtClassDef, Expr, Stmt, StmtAnnAssign, StmtAugAssign, StmtAssign};

#[derive(Default)]
pub struct TypeDefinitionParser {}

impl Parser<WithSource<StmtClassDef>> for TypeDefinitionParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>, config: &ParserConfig) -> Result<Self::Output> {
        let identifier_parser = IdentifierParser::new();
        let identifier = identifier_parser.parse(input.ast.name.as_str(), config)?;
        if config.get_only_parse_symbols() {
            Ok(TypeDefinition { identifier, ..Default::default() })
        } else {
            let attributes = AttributesParser::default().parse(input.sub(&input.ast.decorator_list), config).unwrap_or_default(); // TODO: Maybe we want the signalize the failures.
            let visibility = identifier_parser.get_visibility(&identifier);
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
        let identifier_parser = IdentifierParser::new();
        let identifier = identifier_parser.parse(identifier, config)?;
        let visibility = identifier_parser.get_visibility(&identifier);
        let identifier = Some(identifier);
        let type_ = TypeParser::new().parse(input.sub(&*input.ast.annotation), config)?;
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
        let parser = IdentifierParser::new();
        let identifier = parser.parse(identifier, config)?;
        if let Mutability::Mutable = parser.get_mutability(&identifier) {
            let visibility = parser.get_visibility(&identifier);
            let identifier = Some(identifier);
            let type_ = Default::default();
            let attributes = Default::default();
            Ok(Field { identifier, type_, visibility, attributes })
        } else {
            Err(Error::Message("Expected mutable identifier".into()))
        }
    }

    fn parse_fields_from_assign(&self, input: &WithSource<&StmtAssign>, config: &ParserConfig) -> Result<Vec<Field>> {
        let mut fields = Vec::new();
        for target in &input.ast.targets {
            if let Some(identifier) = target.as_name_expr() {
                let parser = IdentifierParser::new();
                let identifier = parser.parse(identifier.id.as_str(), config)?;
                if let Mutability::Mutable = parser.get_mutability(&identifier) {
                    let visibility = parser.get_visibility(&identifier);
                    let identifier = Some(identifier);
                    let type_ = Default::default();
                    let attributes = Default::default();
                    let field = Field { identifier, type_, visibility, attributes };
                    fields.push(field);
                }
            }
        }
        Ok(fields)
    }

    fn parse_kind_definition(&self, input: &WithSource<StmtClassDef>, config: &ParserConfig) -> Result<KindDefinition> {
        let mut fields = Vec::new();
        let class_variables_as_properties = PythonParserConfig::from(config).get_class_variables_as_properties();
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
                    if function.attributes.contains("property") {
                        let identifier = Some(function.identifier);
                        let type_ = function.output.unwrap_or_default();
                        let field = Field { identifier, type_, ..Default::default() };
                        fields.push(field);
                    }
                },
                _ => ()
            }
        }
        let mut set = HashMap::new();
        let duplicated_fields = fields
            .into_iter()
            .map(|field| set.insert(field.identifier.clone(), field))
            .collect::<Vec<_>>();
        for field in duplicated_fields.into_iter().flatten() {
            let stored = set.get(&field.identifier).unwrap();
            if stored.type_ == Type::opaque() {
                set.insert(stored.identifier.clone(), field);
            }
        }
        let fields = set.into_values().collect();
        let structure = Structure { fields };
        Ok(structure.into())
    }
}
