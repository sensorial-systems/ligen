/// Reference type iterator.
pub struct TypeIterator<'a, Value>
{
    stack: Vec<&'a Value>,
}

impl<'a, Value> From<Vec<&'a Value>> for TypeIterator<'a, Value> {
    fn from(stack: Vec<&'a Value>) -> Self {
        Self { stack }
    }
}

pub trait TypeIter<Item> {
    fn type_iterator(&self) -> TypeIterator<'_, Item>;
}

impl<'a, Value> Iterator for TypeIterator<'a, Value>
{
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}


/// Mutable reference type iterator.

pub struct TypeIterMut<'a, Value>
{
    stack: Vec<&'a mut Value>,
}

impl<'a, Value> From<Vec<&'a mut Value>> for TypeIterMut<'a, Value> {
    fn from(stack: Vec<&'a mut Value>) -> Self {
        Self { stack }
    }
}

pub trait IntoIterTypeMut<Item> {
    fn type_iterator(&mut self) -> TypeIterMut<'_, Item>;
    
}

impl<'a, Value> Iterator for TypeIterMut<'a, Value>
{
    type Item = &'a mut Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

/// Type iterator trait.
pub trait IterType {
    fn iter_type<T>(&self) -> TypeIterator<'_, T>
    where Self: TypeIter<T>
    {
        self.type_iterator()
    }
}

pub trait IterTypeMut {
    fn iter_type_mut<T>(&mut self) -> TypeIterMut<'_, T>
    where Self: IntoIterTypeMut<T>
    {
        self.type_iterator()
    }
}

impl<T> IterType for T {}
impl<T> IterTypeMut for T {}
