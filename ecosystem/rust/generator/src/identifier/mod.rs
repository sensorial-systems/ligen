use ligen_transformer::prelude::*;
use ligen_idl::Identifier;

#[derive(Default)]	
pub struct RustIdentifierGenerator;

impl Generator<&Identifier, syn::Ident> for RustIdentifierGenerator {
    fn generate(&self, identifier: &Identifier, _config: &Config) -> Result<syn::Ident> {
        let name =
        if identifier == &Identifier::u8() || identifier == &Identifier::u16() || identifier == &Identifier::u32() || identifier == &Identifier::u64() || identifier == &Identifier::u128()
        || identifier == &Identifier::i8() || identifier == &Identifier::i16() || identifier == &Identifier::i32() || identifier == &Identifier::i64() || identifier == &Identifier::i128() 
        || identifier == &Identifier::f32() || identifier == &Identifier::f64()
        {
            &identifier.to_string().to_lowercase()
        } else {
            &identifier.to_string()
        };

        Ok(syn::Ident::new(name, proc_macro2::Span::call_site()))
    }
}
