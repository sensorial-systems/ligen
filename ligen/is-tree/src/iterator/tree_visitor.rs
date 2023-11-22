use std::rc::Rc;

use crate::{HasIdentifier, Visitor, IsTree};

pub struct TreeVisitor<'a, Value>
where Value: HasIdentifier,
{
    stack: Vec<Rc<Visitor<'a, Value>>>,
}

impl<'a, Value> TreeVisitor<'a, Value>
where
    Value: HasIdentifier + IsTree,
{
    pub fn new(root: &'a Value) -> Self {
        let visitor = Visitor::new(root, Default::default(), Default::default());
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(visitor);
        iterator
    }

    fn build(&mut self, visitor: Rc<Visitor<'a, Value>>) {
        self.stack.push(visitor.clone());
        for child in visitor.value.branches() {
            let visitor = visitor.child(child);
            self.build(visitor);
        }
    }
}

impl<'a, Value> Iterator for TreeVisitor<'a, Value>
where Value: HasIdentifier
{
    type Item = Rc<Visitor<'a, Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
