mod attribute;

pub use attribute::*;

use crate::literal::LiteralParser;
pub type AttributesParser = ligen::parser::universal::attributes::AttributesParser<LiteralParser>;

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;
    use ligen::ir::attributes::mock;
    use ligen::parser::assert::assert_eq;

    #[test]
    fn parse_literals() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_literals(), "c(marshal_as(name = \"hello\", uuid = 5), int = \"sized\")")
    }

    #[test]
    fn parse_attributes() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_attributes(), "c(int = \"sized\")")
    }

    #[test]
    fn parse_expressions() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_expressions(), r#"error("the {} field name: '{}' is invalid, path: {:?}", self.0.field_type, self.0.field_name, self.0.path)"#) // we need to make expressions valid.
    }
}
