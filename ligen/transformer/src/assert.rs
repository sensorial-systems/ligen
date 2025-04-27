use crate::parser::*;
use ligen_common::Result;
pub use pretty_assertions::assert_eq;

pub fn assert_eq<P, I>(parser: P, expected: P::Output, actual: I) -> Result<()>
    where P: Parser<I>,
          P::Output: std::fmt::Debug + PartialEq
{
    assert_eq!(expected, parser.parse(actual, &Default::default())?);
    Ok(())
}

pub fn assert_failure<P, I>(parser: P, actual: I) -> Result<()>
    where P: Parser<I>
{
    assert!(parser.parse(actual, &Default::default()).is_err());
    Ok(())
}