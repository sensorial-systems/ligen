use std::collections::HashSet;

use ligen_ir::Identifier;
use ligen_utils::mapper::LanguageMap;

pub struct IdentifierGenerator {
    map: LanguageMap<Identifier>,
    keywords: HashSet<Identifier>
}

impl Default for IdentifierGenerator {
    fn default() -> Self {
        let mut keywords = HashSet::new();
        keywords.insert("type".into());
        keywords.insert("box".into());
    
        let mut map = LanguageMap::new("ligen", "rust");
        map.insert(Identifier::boolean(), "bool");
        map.insert(Identifier::i8(), "i8");
        map.insert(Identifier::i16(), "i16");
        map.insert(Identifier::i32(), "i32");
        map.insert(Identifier::i64(), "i64");
        map.insert(Identifier::i128(), "i128");
        map.insert(Identifier::u8(), "u8");
        map.insert(Identifier::u16(), "u16");
        map.insert(Identifier::u32(), "u32");
        map.insert(Identifier::u64(), "u64");
        map.insert(Identifier::u128(), "u128");
        map.insert(Identifier::f32(), "f32");
        map.insert(Identifier::f64(), "f64");
        map.insert(Identifier::character(), "char");
        map.insert(Identifier::string(), "String");
        map.insert(Identifier::option(), "Option");
        map.insert(Identifier::date_time(), "pyo3::Py<pyo3::types::PyDateTime>");
        map.insert(Identifier::vector(), "Vec");
        map.insert(Identifier::opaque(), "pyo3::PyObject");
        map.insert(Identifier::dictionary(), "pyo3::Py<pyo3::types::PyDict>");
        map.insert(Identifier::tuple(), "pyo3::Py<pyo3::types::PyTuple>");
        Self { map, keywords }
    }
}

impl IdentifierGenerator {
    pub fn translate(&self, identifier: &Identifier) -> Identifier {
        let identifier = self
            .map
            .get("ligen", identifier)
            .unwrap_or(identifier).clone();
        let identifier = self
                .keywords
                .get(&identifier)
                .map(|identifier| {
                    let mut identifier = identifier.clone();
                    identifier.name = format!("{}_", identifier.name);
                    identifier
                }).unwrap_or_else(|| identifier.clone());
        identifier
    }
}