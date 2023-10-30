use crate::{prelude::*, identifier::IdentifierParser, macro_attributes::attributes::AttributesParser, function::FunctionParser};
use ligen::ir::{TypeDefinition, Visibility, Path, KindDefinition, Structure, Attribute, Field};
use rustpython_parser::ast::{StmtClassDef, Expr};

use super::DynamicParser;

#[derive(Default)]
pub struct FullParser;

impl<'a> DynamicParser<'a> for FullParser {}

impl Parser<WithSource<StmtClassDef>> for FullParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(input.sub(input.ast.decorator_list.clone()))?;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let visibility = Visibility::Public;
        let interfaces = self.parse_interfaces(&input.ast.bases)?;
        let definition = self.parse_kind_definition(&input)?;
        Ok(TypeDefinition { attributes, visibility, identifier, definition, interfaces })
    }
}

impl FullParser {
    fn parse_interfaces(&self, input: &Vec<Expr>) -> Result<Vec<Path>> {
        let mut interfaces = Vec::new();
        for expr in input {
            if let Some(expr) = expr.as_name_expr() {
                interfaces.push(IdentifierParser::default().parse(expr.id.as_str())?.into());
            }
        }
        Ok(interfaces)
    }

    fn parse_kind_definition(&self, input: &WithSource<StmtClassDef>) -> Result<KindDefinition> {
        let mut fields = Vec::new();
        for stmt in &input.ast.body {
            if let Some(function_def) = stmt.as_function_def_stmt() {
                let function = FunctionParser::full().parse(input.sub(function_def.clone())).expect("Aqui");
                if function.attributes.contains(&Attribute::Group("property".into(), Default::default())) {
                    let identifier = Some(function.identifier);
                    let type_ = function.output.unwrap_or_default();
                    let field = Field { identifier, type_, ..Default::default() };
                    fields.push(field);
                }
            }
        }
        let structure = Structure { fields };
        Ok(structure.into())
    }
}
