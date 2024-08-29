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

    // TODO: Finish this test
    // #[test]
    // fn parse_many_attributes() -> Result<()> {
    //     assert_eq(AttributesParser::default(), mock::parse_many_attributes(), "error(\"the {} field name: '{}' is invalid, path: {:?}\", .0.field_type, .0.field_name, .0.path)")
    // }
}
