use std::pin::Pin;

mod path_tree;

pub use path_tree::*;

pub struct Tree<'a, T: 'a> {
    pub parent: Option<&'a Tree<'a, T>>,
    pub data: T,
    pub children: Vec<Pin<Box<Tree<'a, T>>>>
}

impl<'a, T: 'a> Tree<'a, T> {
    pub fn new<Data: Into<T>>(data: Data) -> Pin<Box<Self>> {
        let parent = None;
        let data = data.into();
        let children = Default::default();
        Box::pin(Self { parent, data, children })
    }

    pub fn add_child(&'a self, mut tree: Pin<Box<Self>>) {
        unsafe {
            let mut_tree: Pin<&mut Self> = Pin::as_mut(&mut tree);
            Pin::get_unchecked_mut(mut_tree).parent = Some(self);
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            let mut_self = &mut *mut_ptr;
            mut_self.children.push(tree);
        }
    }

    pub fn get_root(&'a self) -> &'a Tree<'a, T> {
        self
            .parent
            .map(|parent| parent.get_root())
            .unwrap_or(self)
    }
}
