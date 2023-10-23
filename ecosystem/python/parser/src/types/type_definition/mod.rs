use crate::prelude::*;
use ligen::parsing::dynamic_parser;
use ligen::ir::TypeDefinition;
use rustpython_parser::ast::StmtClassDef;

mod full_parser;
mod symbol_parser;

dynamic_parser! {
    TypeDefinitionParser,
    full_parser::FullParser,
    symbol_parser::SymbolParser,
    TypeDefinition,
    WithSource<StmtClassDef>
}