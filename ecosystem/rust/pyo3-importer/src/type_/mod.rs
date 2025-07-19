use ligen::idl::Type;

use crate::identifier::IdentifierGenerator;

#[derive(Default)]
pub struct TypeGenerator {
    identifier_generator: IdentifierGenerator
}

impl TypeGenerator {
    pub fn translate(&self, type_: &Type) -> Type {
        let mut path = type_.path.clone();
        path.segments.iter_mut().for_each(|segment| {
            let identifier = self.identifier_generator.translate(&segment.identifier);
            segment.identifier = identifier;
            segment.generics.types.iter_mut().for_each(|type_| *type_ = self.translate(type_));
        });
        path.into()
    }

}