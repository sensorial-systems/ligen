use crate::ir::{Attribute, Attributes, Identifier, Literal};

impl Attribute {
    /// Convert Attribute to a Ligen Macro attribute
    pub fn to_ligen_macro(&self) -> Self {
        match self.clone() {
            Attribute::Literal(literal) => {
                Self::Literal(Literal::String(format!("ligen_{}", literal.to_string())))
            }
            Attribute::Named(ident, lit) => Self::Named(ident, lit),
            Attribute::Group(ident, group) => Self::Group(
                Identifier::new(format!("ligen_{}", ident.name).as_str()),
                Attributes {
                    attributes: group
                        .attributes
                        .into_iter()
                        .filter_map(|x| {
                            if let Attribute::Named(ident, lit) = x {
                                Some(Attribute::Named(ident, lit))
                            } else {
                                None
                            }
                        })
                        .collect(),
                },
            ),
        }
    }
}
