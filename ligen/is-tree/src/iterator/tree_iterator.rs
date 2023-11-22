use crate::IsTree;

/// Reference iterator.
pub struct TreeIterator<'a, Value> {
    stack: Vec<&'a Value>,
}

impl<'a, Value> TreeIterator<'a, Value>
where
    Value: IsTree,
{
    pub fn new(root: &'a Value) -> Self {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(root);
        iterator
    }

    fn build(&mut self, value: &'a Value) {
        self.stack.push(value);
        for child in value.branches() {
            self.build(child);
        }
    }
}

impl<'a, Value> Iterator for TreeIterator<'a, Value> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

/// Mutable reference iterator.
pub struct TreeIteratorMut<'a, Value> {
    stack: Vec<&'a mut Value>,
}

impl<'a, Value> TreeIteratorMut<'a, Value>
where
    Value: IsTree,
{
    pub fn new(root: &'a mut Value) -> Self {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(root);
        iterator
    }

    fn build(&mut self, value: &'a mut Value) {
        // FIXME: Is it safe?
        let same_value = unsafe { &mut *(value as *mut Value) };
        self.stack.push(same_value);
        for child in value.branches_mut() {
            self.build(child);
        }
    }
}

impl<'a, Value> Iterator for TreeIteratorMut<'a, Value> {
    type Item = &'a mut Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
