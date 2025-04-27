use ligen_transformer::prelude::*;
use ligen_ir::Identifier;

#[derive(Default)]	
pub struct RustIdentifierGenerator;

impl Generator<&Identifier, syn::Ident> for RustIdentifierGenerator {
    fn generate(&self, identifier: &Identifier, _config: &Config) -> Result<syn::Ident> {
        Ok(syn::Ident::new(&identifier.to_string(), proc_macro2::Span::call_site()))
    }
}
