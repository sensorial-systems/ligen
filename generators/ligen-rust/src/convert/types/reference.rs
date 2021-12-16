use ligen_ir::ReferenceKind;
use crate::prelude::*;
use ligen_ir::Reference;
use crate::traits::AsRust;

impl AsRust for Reference {
    fn as_rust(&self) -> String {
        let mut string = String::new();
        match self.kind {
            ReferenceKind::Pointer => {
                if self.is_constant {
                    string.push_str("*const ");
                } else {
                    string.push_str(("*mut ");
                }
            },
            ReferenceKind::Borrow => {
                if self.is_constant {
                    string.push_str(("&");
                } else {
                    string.push_str(("&mut ");
                }
            }
        }
        string.push_str(&self.type_.as_rust());
        string
    }
}