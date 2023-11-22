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

pub trait IntoIterType<Item> {
    fn into_type_iterator<'a>(&'a self) -> TypeIterator<'a, Item>;
}

impl<'a, Value> Iterator for TypeIterator<'a, Value>
{
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}


/// Mutable reference type iterator.

pub struct TypeIteratorMut<'a, Value>
{
    stack: Vec<&'a mut Value>,
}

impl<'a, Value> From<Vec<&'a mut Value>> for TypeIteratorMut<'a, Value> {
    fn from(stack: Vec<&'a mut Value>) -> Self {
        Self { stack }
    }
}

pub trait IntoIterTypeMut<Item> {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Item>;
    
}

impl<'a, Value> Iterator for TypeIteratorMut<'a, Value>
{
    type Item = &'a mut Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

/// Type iterator trait.
pub trait IterType {
    fn iter_type<'a, T>(&'a self) -> TypeIterator<'a, T>
    where Self: IntoIterType<T>
    {
        self.into_type_iterator()
    }
}

pub trait IterTypeMut {
    fn iter_type_mut<'a, T>(&'a mut self) -> TypeIteratorMut<'a, T>
    where Self: IntoIterTypeMut<T>
    {
        self.into_type_iterator()
    }
}

impl<T> IterType for T {}
impl<T> IterTypeMut for T {}
