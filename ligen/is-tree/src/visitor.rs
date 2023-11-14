use crate::{HasIdentifier, Path};

pub struct Visitor<'a, Value>
where Value: HasIdentifier
{
    pub value: Value,
    pub path: Path<'a, Value::Identifier>
}

impl<'a, Value> Visitor<'a, Value>
where Value: HasIdentifier
{
    pub fn new(value: Value, path: Path<'a, Value::Identifier>) -> Self {
        Self { value, path }
    }
}
