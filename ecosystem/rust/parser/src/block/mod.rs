use ligen::parser::universal::PathParser;
use ligen::prelude::*;
use ligen::ir::{Block, Statement, Expression, BinaryExpression};
use quote::quote;


#[derive(Default)]
pub struct RustBlockParser {
    path_parser: PathParser,
}

impl Transformer<Box<syn::Block>, Option<Block>> for RustBlockParser {
    fn transform(&self, block: Box<syn::Block>, config: &Config) -> Result<Option<Block>> {
        self.transform(*block, config)
    }
}

impl Transformer<syn::Block, Option<Block>> for RustBlockParser {
    fn transform(&self, block: syn::Block, config: &Config) -> Result<Option<Block>> {
        Ok(if block.stmts.is_empty() {
            None
        } else {
            Some(self.transform(block, config)?)
        })
    }
}

impl Transformer<syn::Block, Block> for RustBlockParser {
    fn transform(&self, block: syn::Block, config: &Config) -> Result<Block> {
        let mut statements = Vec::new();
        for stmt in block.stmts {
            statements.push(self.transform(stmt, config)?);
        }
        Ok(Block::new(statements))
    }
}

impl Transformer<syn::Stmt, Statement> for RustBlockParser {
    fn transform(&self, stmt: syn::Stmt, config: &Config) -> Result<Statement> {
        match stmt {
            syn::Stmt::Expr(expr, _) => {
                match expr {
                    syn::Expr::Return(expr) => {
                        if let Some(expr) = expr.expr {
                            let expr = self.transform(expr, config)?;
                            Ok(Statement::return_(Some(expr)))
                        } else {
                            Ok(Statement::return_(None as Option<Expression>))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Unsupported expression").into()),
                }
            }
            _ => Err(anyhow::anyhow!("Unsupported statement").into()),
        }
    }
}

impl Transformer<Box<syn::Expr>, Expression> for RustBlockParser {
    fn transform(&self, expr: Box<syn::Expr>, config: &Config) -> Result<Expression> {
        self.transform(*expr, config)
    }
}

impl Transformer<syn::Expr, Expression> for RustBlockParser {
    fn transform(&self, expr: syn::Expr, config: &Config) -> Result<Expression> {
        match expr {
            syn::Expr::Binary(binary) => {
                let left = self.transform(binary.left, config)?;
                let right = self.transform(binary.right, config)?;
                let operator = match binary.op {
                    syn::BinOp::Div(_) => "/",
                    _ => return Err(anyhow::anyhow!("Unsupported binary operation").into())
                };
                Ok(BinaryExpression::new(left, operator, right).into())
            },
            syn::Expr::Path(path) => {
                let path = self.path_parser.transform(path.path, config)?;
                Ok(path.into())
            },
            _ => Err(anyhow::anyhow!("Unsupported binary expression: {}", quote! { #expr }).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use ligen::ir::{Statement, Expression, BinaryExpression, Path};

    use super::*;

    #[test]
    fn block() {
        let block_parser = RustBlockParser::default();
        let block = syn::parse_str::<syn::Block>("{
            return a / b;
        }").unwrap();
        let block: Block = block_parser.transform(block, &Config::default()).unwrap();
        assert_eq!(block, Block::new(vec![Statement::return_(Some(Expression::Binary(BinaryExpression::new(Path::from("a"), "/", Path::from("b")))))]));
    }
}