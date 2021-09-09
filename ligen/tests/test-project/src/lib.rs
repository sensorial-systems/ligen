use ligen_macro::ligen;

pub struct Object;

impl Object {
    pub fn new() -> Self {
        Self
    }
}

mod ignored;

#[ligen(ignore)]
pub mod inline_ignored {
    pub struct Ignored;
}