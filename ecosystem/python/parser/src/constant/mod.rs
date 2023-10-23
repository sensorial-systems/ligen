use ligen::parsing::dynamic_parser;
use ligen::ir::Constant;
use rustpython_parser::ast::{StmtAnnAssign, StmtAugAssign, Expr, StmtAssign};

mod full_parser;
mod symbol_parser;

dynamic_parser!{
    ConstantParser,
    full_parser::FullParser,
    symbol_parser::SymbolParser,
    Constant,
    &StmtAnnAssign | &'a StmtAnnAssign,
    &StmtAugAssign | &'a StmtAugAssign,
    &Expr | &'a Expr,
    &StmtAssign | &'a StmtAssign => Vec<Constant>
}
