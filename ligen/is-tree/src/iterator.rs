use crate::{HasIdentifier, Visitor, IsTree};

pub struct TreeIterator<'a, Value>
where Value: HasIdentifier,
{
    stack: Vec<Visitor<'a, Value>>,
}

impl<'a, Value> TreeIterator<'a, Value>
where
    Value: HasIdentifier,
{
    pub fn new(root: &'a Value) -> Self {
        let mut stack = Vec::new();
        stack.push(Visitor::new(root, Default::default()));
        Self { stack }
    }
}

impl<'a, Value> Iterator for TreeIterator<'a, Value>
where Value: HasIdentifier + IsTree
{
    type Item = Visitor<'a, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.stack.extend(node.value.branches().map(|branch| {
            let mut path = node.path.clone();
            path.segments.push(branch.identifier().clone());
            Visitor::new(branch, path)
        }));
        Some(node)
    }
}