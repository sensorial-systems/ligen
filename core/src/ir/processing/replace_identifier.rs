//! Trait for replacing an existing identifier.
// TODO: Generalize it for any AST element.

use crate::ir::{Identifier, Implementation, ImplementationItem, Type, Reference, Parameter};

/// Trait to replace identifiers in IR AST.
pub trait ReplaceIdentifier {
    /// Replace all occurrences of the `old` identifier by the `new` identifier.
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier);
}

impl ReplaceIdentifier for Implementation {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        for item in &mut self.items {
            item.replace_identifier(old, new);
        }
    }
}

impl ReplaceIdentifier for ImplementationItem {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        match self {
            ImplementationItem::Method(function) => {
                function.identifier.replace_identifier(old, new);
                function.output.as_mut().map(|type_| type_.replace_identifier(old, new));
                for parameter in &mut function.input {
                    parameter.replace_identifier(old, new);
                }
            },
            ImplementationItem::Constant(constant) => {
                constant.identifier.replace_identifier(old, new);
            }
        }
    }
}

impl ReplaceIdentifier for Parameter {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        self.identifier.replace_identifier(old, new);
        self.type_.replace_identifier(old, new);
    }
}

impl ReplaceIdentifier for Type {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        match self {
            Type::Reference(reference) => {
                reference.replace_identifier(old, new);
            },
            Type::Compound(compound) => {
                compound.replace_identifier(old, new);
            },
            _ => ()
        }
    }
}

impl ReplaceIdentifier for Reference {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        self.type_mut().replace_identifier(old, new);
    }
}

impl ReplaceIdentifier for Identifier {
    fn replace_identifier(&mut self, old: &Identifier, new: &Identifier) {
        self.name = self.name.replace(&old.name, &new.name);
    }
}