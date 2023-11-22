use std::rc::Rc;

use crate::{IsIdentifier, HasIdentifier, Path, Identifier, IsTree};

pub struct Visitor<'a, Value>
where Value: HasIdentifier
{
    pub parent: Option<Rc<Visitor<'a, Value>>>,
    pub value: &'a Value,
    pub path: Path<'a, Value::Identifier>
}

impl<'a, Value> Visitor<'a, Value>
where Value: HasIdentifier
{
    pub fn new(value: &'a Value, parent: Option<Rc<Visitor<'a, Value>>>, path: Path<'a, Value::Identifier>) -> Rc<Self> {
        Rc::new(Self { value, parent, path })
    }

    pub fn child(self: &Rc<Self>, value: &'a Value) -> Rc<Self>
    {
        let path = self.path.join(value.identifier().clone());
        let child = Self::new(value, Some(self.clone()), path);
        child
    }

    pub fn root(self: &Rc<Self>) -> Rc<Self> {
        self.parent
            .as_ref()
            .map(|parent| parent.root())
            .unwrap_or(self.clone())
    }

    pub fn relative<K>(self: &Rc<Self>, path: impl IntoIterator<Item = K>) -> Option<Rc<Self>>
    where K: Into<Value::Identifier>,
        Value: IsTree
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                Identifier::Root => Some(self.root()),
                Identifier::Self_ => self.relative(path),
                Identifier::Super => self
                    .parent
                    .as_ref()
                    .and_then(|parent| parent.relative(path)),
                Identifier::Other(segment) => self
                    .value
                    .get(segment.clone())
                    .and_then(|branch|
                        self.child(branch)
                            .relative(path)
                    )
            }
        } else {
            Some(self.clone())
        }
    }
}

