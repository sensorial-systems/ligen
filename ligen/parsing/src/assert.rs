use crate::Parser;
use ligen_common::Result;

pub fn assert_eq<P, I>(parser: P, expected: P::Output, actual: I) -> Result<()>
    where P: Parser<I>,
          P::Output: std::fmt::Debug + PartialEq
{
    assert_eq!(expected, parser.parse(actual)?);
    Ok(())
}