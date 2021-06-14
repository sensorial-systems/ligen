//! Source files generator.

// use crate::ir::Implementation;
// use std::path::Path;
// use std::collections::HashMap;
//
// pub struct SourceSet(HashMap<Path, String>);
//
// impl SourceSet {
//     // FIXME: We want to create the value if it doesn't exist.
//     pub fn get(&self, path: &Path) -> Option<&String> {
//         self.0.get(path)
//     }
//
//     pub fn get_mut(&mut self, path: &Path) -> Option<&mut String> {
//         self.0.get_mut(path)
//     }
// }
//
// pub struct SourceFile {
//     pub path: Path,
//     pub content: String
// }
//
// pub trait Generator {
//     fn generate(&self) -> SourceSet;
// }