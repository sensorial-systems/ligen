use crate::parser::Parser;
use ligen_ir::Literal;

pub trait LiteralParser: Parser<String, Output = Literal> + Default {}
