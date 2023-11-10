use crate::prelude::*;
use std::fmt::Display;

pub trait Identifier {
    type Identifier: PartialEq;
    fn identifier(&self) -> &Self::Identifier;
}

pub struct Path<Segment>
{
    pub segments: Vec<Segment>
}

impl<Segment> From<Segment> for Path<Segment> {
    fn from(value: Segment) -> Self {
        let segments = vec![value];
        Self { segments }
    }
}

impl<Segment> From<Vec<Segment>> for Path<Segment> {
    fn from(segments: Vec<Segment>) -> Self {
        Self { segments }
    }
}

impl<Segment> From<&[Segment]> for Path<Segment>
where Segment: Clone
{
    fn from(segments: &[Segment]) -> Self {
        let segments = segments.to_vec();
        Self { segments }
    }
}

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

    pub fn get<Segment>(&self, path: impl Into<Path<Segment>>) -> Option<&Self>
    where Segment: PartialEq<Value::Identifier> + Copy + Display,
    Value::Identifier: Display
    {
        let path = path.into();
        if let Some(identifier) = path.segments.first() {
            println!("{} - {}", self.identifier(), identifier);
            if self.is(*identifier) {
                Some(self)
            } else {
                self
                .children
                .iter()
                .find_map(|child| child.get::<Segment>(&path.segments[1..]))
            }
        } else {
            None
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree() {
        let tree = Tree::from(5);
        assert!(tree.is(5));

        let mut jose = Tree::from(Person::from("José"));
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        let danilo = jose.add_child(Person::from("Danilo"));
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");
        
        let joaquim = jose.add_child(Person::from("Joaquim"));
        assert!(joaquim.is("Joaquim"));
        assert_eq!(joaquim.say(), "My name is Joaquim");

        let jose = jose.get("José").unwrap();
        assert!(jose.is("José"));
        assert_eq!(jose.say(), "My name is José");

        println!("Getting Danilo from José.");
        let danilo = jose.get("Danilo").unwrap();
        assert!(danilo.is("Danilo"));
        assert_eq!(danilo.say(), "My name is Danilo");

        println!("Getting Joaquim from José.");
        assert!(jose.get("Joaquim").is_none());
        let joaquim = jose.get::<&str>(["Danilo", "Joaquim"].as_slice()).unwrap();
        assert!(joaquim.is("Joaquim"));
        assert_eq!(joaquim.say(), "My name is Joaquim");
    }
}