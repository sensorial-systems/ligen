use ligen::parsing::dynamic_parser;
use ligen::ir::Object;
use rustpython_parser::ast::{StmtAnnAssign, StmtAugAssign, Expr, StmtAssign};

mod full_parser;
mod symbol_parser;

dynamic_parser!{
    ObjectParser,
    full_parser::FullParser,
    symbol_parser::SymbolParser,
    Object,
    &StmtAnnAssign | &'a StmtAnnAssign,
    &StmtAugAssign | &'a StmtAugAssign,
    &Expr | &'a Expr,
    &StmtAssign | &'a StmtAssign => Vec<Object>
}
