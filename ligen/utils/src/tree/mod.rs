pub mod identifier;
pub mod path;

use std::{collections::HashMap, borrow::{Borrow, BorrowMut}};

pub use identifier::*;
pub use path::*;

use crate::prelude::*;

pub trait TreeTrait<Value>: Identifier {
    fn is(&self, identifier: impl PartialEq<Self::Identifier>) -> bool;

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
    
    // fn path_get<'a, I: Into<Self::Identifier> + 'a>(&'a self, path: impl IntoIterator<Item = &'a I>) -> Option<&Self> {
    //     let mut iter = path.into_iter().peekable();
    //     if let Some(identifier) = iter.next() {
    //         if iter.peek().is_none() && self.is(identifier) {
    //             Some(self)
    //         } else {
    //             self.get(identifier)
    //         }
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Tree<Value>
where Value: Identifier
{
    #[shrinkwrap(main_field)]
    pub value: Value,
    pub children: HashMap<Value::Identifier, Tree<Value>>
}

impl<Value> Identifier for Tree<Value>
where Value: Identifier
{
    type Identifier = Value::Identifier;
    fn identifier(&self) -> &Self::Identifier {
        self.value.identifier()
    }
}

impl<Value> TreeTrait<Value> for Tree<Value>
where Value: Identifier
{
    fn is(&self, identifier: impl PartialEq<Self::Identifier>) -> bool {
        identifier.eq(self.identifier())
    }

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
where Value: Identifier
{
    fn from(value: Value) -> Self {
        let children = Default::default();
        Self { value, children }
    }
}

impl Identifier for usize {
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
    
    impl Identifier for Person {
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

    // #[test]
    // fn get_from_path() {
    //     let jose = create();
    //     let jose = jose.path_get(["José"]).unwrap();
    //     assert!(jose.is("José"));
    //     assert_eq!(jose.say(), "My name is José");        

    //     let danilo = jose.path_get(["José", "Danilo"]).unwrap();
    //     assert!(danilo.is("Danilo"));
    //     assert_eq!(danilo.say(), "My name is Danilo");
    // }
}
