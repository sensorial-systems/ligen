pub mod identifier;
pub mod path;

pub use identifier::*;
pub use path::*;

use crate::prelude::*;
use std::fmt::{Display, Debug};

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Tree<Value>
where Value: Identifier
{
    #[shrinkwrap(main_field)]
    pub value: Value,
    pub children: Vec<Tree<Value>>
}

impl<Value> Tree<Value>
where Value: Identifier
{
    pub fn is(&self, identifier: impl PartialEq<Value::Identifier>) -> bool {
        identifier.eq(self.identifier())
    }

    pub fn add_child(&mut self, child: impl Into<Self>) -> &mut Self {
        self.children.push(child.into());
        self.children.last_mut().unwrap()
    }

    // pub fn get<'a, Segment>(&self, path: impl Into<Path<'a, Segment>>) -> Option<&Self>
    // where Segment: PartialEq<Value::Identifier> + 'a + Display + Debug,
    // Value::Identifier: Display
    // {
    //     let path = path.into();
    //     if let Some(identifier) = path.segments.first() {
    //         let rest = &path.segments[1..];
    //         if rest.is_empty() && self.is(**identifier) {
    //             Some(self)
    //         } else {
    //             self
    //             .children
    //             .iter()
    //             .find_map(|child| child.get::<&Segment>(rest))
    //         }
    //     } else {
    //         None
    //     }
    // }
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
        assert!(tree.is(5));

        let mut jose = Tree::from(Person::from("José"));
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        let danilo = jose.add_child(Person::from("Danilo"));
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");
        
        let joaquim = danilo.add_child(Person::from("Joaquim"));
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
        // let jose = create();
        // let jose = jose.get("José").unwrap();
        // assert!(jose.is("José"));
        // assert_eq!(jose.say(), "My name is José");

        // println!("Getting Danilo from José.");
        // let danilo = jose.get::<&str>([&"José", &"Danilo"].as_slice()).unwrap();
        // assert!(danilo.is("Danilo"));
        // assert_eq!(danilo.say(), "My name is Danilo");

        // println!("Getting Joaquim from José.");
        // assert!(jose.get(&"Joaquim").is_none());
        // let joaquim = jose.get(Path::from(&["José", "Danilo", "Joaquim"])).unwrap();
        // assert!(joaquim.is("Joaquim"));
        // assert_eq!(joaquim.say(), "My name is Joaquim");
    }
}