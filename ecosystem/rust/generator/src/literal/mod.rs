use ligen_transformer::prelude::*;
use ligen_ir::Literal;

#[derive(Default)]	
pub struct RustLiteralGenerator;

impl Generator<&Literal, syn::Lit> for RustLiteralGenerator {
    fn generate(&self, literal: &Literal, _config: &Config) -> Result<syn::Lit> {
        match literal {
            Literal::String(s) => Ok(syn::Lit::Str(syn::LitStr::new(&s.to_string(), proc_macro2::Span::call_site()))),
            Literal::Integer(i) => Ok(syn::Lit::Int(syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site()))),
            Literal::Float(f) => Ok(syn::Lit::Float(syn::LitFloat::new(&f.to_string(), proc_macro2::Span::call_site()))),
            Literal::Boolean(b) => Ok(syn::Lit::Bool(syn::LitBool::new(*b, proc_macro2::Span::call_site()))),
            Literal::Character(c) => Ok(syn::Lit::Char(syn::LitChar::new(*c, proc_macro2::Span::call_site()))),
            _ => Err(Error::Message("Unsupported literal type".to_string())),
        }
    }
}
