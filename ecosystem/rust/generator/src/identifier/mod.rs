use ligen_generator::prelude::*;
use ligen_ir::Identifier;

#[derive(Default)]	
pub struct RustIdentifierGenerator;

impl Generator<Identifier> for RustIdentifierGenerator {
    type Output = syn::Ident;
    fn generate(&self, identifier: &Identifier, _config: &Config) -> Result<syn::Ident> {
        Ok(syn::Ident::new(&identifier.to_string(), proc_macro2::Span::call_site()))
    }
}
