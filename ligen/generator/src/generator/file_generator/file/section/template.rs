use ligen_utils::tree::{IsTree, HasIdentifier};
use std::borrow::Borrow;

pub struct SectionTemplate {
    pub name: String,
    pub content: String,
    children: Vec<Self>
}

impl SectionTemplate {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        let name = name.into();
        let content = content.into();
        let children = Default::default();
        Self { name, content, children }
    }
}

impl HasIdentifier for SectionTemplate {
    type Identifier = String;
    fn identifier(&self) -> &Self::Identifier {
        &self.name
    }
}

impl IsTree for SectionTemplate {
    fn add_branch(&mut self, template: impl Into<Self>) -> &mut Self where Self: Sized {
        let template = template.into();
        self.children.push(template);
        self.children.last_mut().unwrap()
    }

    fn get<K>(&self, key: K) -> Option<&Self>
    where K: Into<Self::Identifier>, Self::Identifier: Borrow<Self::Identifier>
    {
        let key = key.into();
        let key = key.borrow();
        self
            .children
            .iter()
            .find(|section| section.name == key)    
    }

    fn get_mut<K>(&mut self, key: K) -> Option<&mut Self>
    where K: Into<Self::Identifier>, Self::Identifier: std::borrow::BorrowMut<Self::Identifier>
    {
        let key = key.into();
        let key = key.borrow();
        self
            .children
            .iter_mut()
            .find(|section| section.name == key)    
    }

    fn branches<'a>(&'a self) -> Box<dyn Iterator<Item = &Self> + 'a> {
        Box::new(self.children.iter())
    }

    fn branches_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Self> + 'a> {
        Box::new(self.children.iter_mut())
    }
}
