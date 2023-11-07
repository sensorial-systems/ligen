//! Attribute enumeration.

use crate::literal::LiteralParser;

pub type AttributeParser = ligen::parser::universal::attributes::attribute::AttributeParser<LiteralParser>;

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;
    use ligen::parser::assert::assert_eq;
    use ligen::ir::attribute::mock;

    #[test]
    fn attribute_literal() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_literal(), "\"c\"")
    }

    #[test]
    fn attribute_named() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_named(), "int = \"sized\"")
    }

    #[test]
    fn attribute_group() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_group(), "c(int = \"sized\")")
    }

    #[test]
    fn attribute_empty_group() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_empty_group(), "c()")?;
        assert_eq(AttributeParser::default(), mock::attribute_empty_group(), "c")
    }
}