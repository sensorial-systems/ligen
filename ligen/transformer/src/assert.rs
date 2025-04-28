use crate::prelude::*;
pub use pretty_assertions::assert_eq;

pub fn assert_eq<T, I, O>(parser: T, expected: O, actual: I) -> Result<()>
    where T: Transformer<I, O>,
          O: std::fmt::Debug + PartialEq
{
    assert_eq!(expected, parser.transform(actual, &Default::default())?);
    Ok(())
}

pub fn assert_failure<T, I, O>(transformer: T, actual: I) -> Result<()>
    where T: Transformer<I, O>,
          O: std::fmt::Debug + PartialEq
{
    assert!(transformer.transform(actual, &Default::default()).is_err());
    Ok(())
}

pub async fn async_assert_eq<T, I, O>(parser: T, expected: O, actual: I) -> Result<()>
    where T: AsyncTransformer<I, O>,
          O: std::fmt::Debug + PartialEq
{
    assert_eq!(expected, parser.transform(actual, &Default::default()).await?);
    Ok(())
}

pub async fn async_assert_failure<T, I, O>(transformer: T, actual: I) -> Result<()>
    where T: AsyncTransformer<I, O>,
          O: std::fmt::Debug + PartialEq
{
    assert!(transformer.transform(actual, &Default::default()).await.is_err());
    Ok(())
}