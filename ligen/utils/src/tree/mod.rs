pub mod identifier;
pub mod path;

use std::{collections::HashMap, borrow::{Borrow, BorrowMut}, fmt::Display};

pub use identifier::*;
pub use path::*;

use crate::prelude::*;

pub trait IsTree: HasIdentifier {
    fn is(&self, identifier: impl PartialEq<Self::Identifier>) -> bool {
        identifier.eq(self.identifier())
    }

    fn add_branch(&mut self, _child: impl Into<Self>) -> &mut Self where Self: Sized {
        self
    }

    fn remove_branch(&mut self, _identifier: &Self::Identifier) {}

    fn branches<'a>(&'a self) -> Box<dyn Iterator<Item = &Self> + 'a>;
    fn branches_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Self> + 'a>;

    fn get<K>(&self, key: K) -> Option<&Self>
    where K: Into<Self::Identifier>, Self::Identifier: Borrow<Self::Identifier>;
    
    fn get_mut<K>(&mut self, key: K) -> Option<&mut Self>
    where K: Into<Self::Identifier>, Self::Identifier: BorrowMut<Self::Identifier>;
    
    fn path_get<'a, K>(&'a self, path: impl IntoIterator<Item = K>) -> Option<&Self>
    where K: Into<Self::Identifier>, Self::Identifier: Borrow<Self::Identifier>,
    Self::Identifier: Display
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            self
                .get(segment)
                .and_then(|branch|
                    branch.path_get(path)
                )
        } else {
            Some(self)
        }
    }
}

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Tree<Value>
where Value: HasIdentifier
{
    #[shrinkwrap(main_field)]
    pub value: Value,
    pub children: HashMap<Value::Identifier, Tree<Value>>
}

impl<Value> HasIdentifier for Tree<Value>
where Value: HasIdentifier
{
    type Identifier = Value::Identifier;
    fn identifier(&self) -> &Self::Identifier {
        self.value.identifier()
    }
}

impl<Value> IsTree for Tree<Value>
where Value: HasIdentifier
{
    fn add_branch(&mut self, child: impl Into<Self>) -> &mut Self
    where Self: Sized
    {
        let child = child.into();
        self.children
            .entry(child.identifier().clone())
            .or_insert(child)
    }

    fn branches<'a>(&'a self) -> Box<dyn Iterator<Item = &Self> + 'a> {
        Box::new(self.children.values())
    }

    fn branches_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Self> + 'a> {
        Box::new(self.children.values_mut())        
    }

    fn get<K>(&self, key: K) -> Option<&Self>
    where K: Into<Self::Identifier>, Self::Identifier: Borrow<Self::Identifier>{
        let key = key.into();
        let key = key.borrow();
        self
            .children
            .get(key)
    }

    fn get_mut<K>(&mut self, key: K) -> Option<&mut Self>
    where K: Into<Self::Identifier>, Self::Identifier: Borrow<Self::Identifier>{
        let key = key.into();
        let key = key.borrow();
        self
            .children
            .get_mut(key)
    }
}

impl<Value> From<Value> for Tree<Value>
where Value: HasIdentifier
{
    fn from(value: Value) -> Self {
        let children = Default::default();
        Self { value, children }
    }
}

impl HasIdentifier for usize {
    type Identifier = usize;
    fn identifier(&self) -> &Self::Identifier {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Person {
        name: String
    }
    
    impl Person {
        pub fn say(&self) -> String {
            format!("My name is {}", self.name)
        }
    }
    
    impl<S: Into<String>> From<S> for Person {
        fn from(name: S) -> Self {
            let name = name.into();
            Self { name }       
        }
    }
    
    impl HasIdentifier for Person {
        type Identifier = String;
        fn identifier(&self) -> &Self::Identifier {
            &self.name
        }
    }

    fn create() -> Tree<Person> {
        let tree = Tree::from(5);
        assert!(tree.is(5 as usize));

        let mut jose = Tree::from(Person::from("José"));
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        let danilo = jose.add_branch(Person::from("Danilo"));
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");
        
        let joaquim = danilo.add_branch(Person::from("Joaquim"));
        assert!(joaquim.is("Joaquim"));
        assert_eq!(joaquim.say(), "My name is Joaquim");

        jose
    }

    #[test]
    fn creation() {
        create();
    }

    #[test]
    fn get() {
        let jose = create();
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        let danilo = jose.get("Danilo").unwrap();
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");

        let joaquim = danilo.get("Joaquim").unwrap();
        assert!(joaquim.is("Joaquim"));
        assert_eq!(joaquim.say(), "My name is Joaquim");
    }

    #[test]
    fn get_from_path() {
        let jose = create();
        let jose = jose.path_get::<&str>([]).unwrap();
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        assert!(jose.path_get(["Ninguém"]).is_none());
        assert!(jose.path_get(["Danilo", "Olívia"]).is_none());

        let danilo = jose.path_get(["Danilo"]).unwrap();
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");

        let joaquim = jose.path_get(["Danilo", "Joaquim"]).unwrap();
        assert!(joaquim.is("Joaquim"));
        assert_eq!(joaquim.say(), "My name is Joaquim");
    }
}
