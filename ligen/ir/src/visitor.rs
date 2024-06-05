use is_tree::visitor;

use crate::{Library, Module};

visitor! {
    pub enum Visitors, VisitorsMut {
        Root(Library visits [Module]),
        Branches(
            Module visits [Module]
        )
    }
}