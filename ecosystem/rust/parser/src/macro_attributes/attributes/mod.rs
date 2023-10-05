mod attribute;

pub use attribute::*;

use crate::literal::LiteralParser;
pub type AttributesParser = ligen::parsing::parser::universal::attributes::AttributesParser<LiteralParser>;

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;
    use ligen::ir::attributes::mock;
    use ligen::parsing::assert::assert_eq;

    #[test]
    fn parse_literals() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_literals(), "c(marshal_as(name = \"hello\", uuid = 5), int = \"sized\")")
    }

    #[test]
    fn parse_attributes() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_attributes(), "c(int = \"sized\")")
    }
}
