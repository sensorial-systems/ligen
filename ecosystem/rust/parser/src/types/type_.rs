use crate::path::RustPathParser;
use crate::{literal::RustLiteralParser, mutability::RustMutabilityParser};
use ligen::idl::Type;
use ligen::prelude::*;
use quote::ToTokens;
use syn::{TypeArray, TypeSlice};

#[derive(Default)]
pub struct RustTypeParser {
    pub mutability_parser: RustMutabilityParser,
    pub literal_parser: RustLiteralParser,
    pub path_parser: RustPathParser,
}

impl RustTypeParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::Ident, Type> for RustTypeParser {
    fn transform(&self, input: syn::Ident, config: &Config) -> Result<Type> {
        Ok(self.path_parser.transform(input, config)?.into())
    }
}

impl Transformer<syn::Path, Type> for RustTypeParser {
    fn transform(&self, path: syn::Path, config: &Config) -> Result<Type> {
        let mut path = self.path_parser.transform(path, config)?;
        if path.segments.len() == 1 {
            let segment = path.first_mut();
            match segment.identifier.name.as_str() {
                "char" => segment.identifier.name = "Character".into(),
                "bool" => segment.identifier.name = "Boolean".into(),
                _ => (),
            }
        }
        Ok(path.into())
    }
}

impl Transformer<syn::Type, Type> for RustTypeParser {
    fn transform(&self, syn_type: syn::Type, config: &Config) -> Result<Type> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = syn_type {
            Ok(self.transform(path, config)?)
        } else {
            match syn_type {
                syn::Type::Reference(syn::TypeReference { elem, mutability, .. }) |
                syn::Type::Ptr(syn::TypePtr { elem, mutability, .. }) => {
                    let mutability = self.mutability_parser.transform(mutability, config)?;
                    let type_ = RustTypeParser::new().transform(*elem, config)?;
                    Ok(Type::reference(mutability, type_))
                },
                syn::Type::Slice(TypeSlice { elem, .. }) => {
                    let type_ = RustTypeParser::new().transform(*elem, config)?;
                    Ok(Type::slice(type_))
                },
                syn::Type::Array(TypeArray { elem, len, .. }) => {
                    let len = self.literal_parser.transform(len, config)?;
                    let len = len.into_integer().map_err(|_| Error::Message("Array length literal isn't an integer.".into()))? as usize;
                    let type_ = RustTypeParser::new().transform(*elem, config)?;
                    Ok(Type::array(type_, len))
                },
                syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                    let types = elems.into_iter().map(|elem| self.transform(elem, config)).collect::<Result<Vec<_>>>()?;
                    Ok(Type::tuple(types))
                },
                syn::Type::Paren(syn::TypeParen { elem, .. }) |
                syn::Type::Group(syn::TypeGroup { elem, .. }) => {
                    self.transform(*elem, config)
                },
                syn::Type::BareFn(bare_fn) => {
                    let inputs = bare_fn.inputs
                        .iter()
                        .map(|arg| self.transform(arg.ty.clone(), config))
                        .collect::<Result<Vec<_>>>()?;
                    let output = match bare_fn.output {
                        syn::ReturnType::Default => Type::void(),
                        syn::ReturnType::Type(_, elem) => self.transform(*elem, config)?,
                    };
                    Ok(Type::function(inputs, output))
                },
                syn::Type::Never(_) => {
                    Ok(Type::void())
                },
                syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) |
                syn::Type::ImplTrait(syn::TypeImplTrait { bounds, .. }) => {
                    if let Some(syn::TypeParamBound::Trait(trait_bound)) = bounds.first() {
                        self.path_parser.transform(trait_bound.path.clone(), config).map(Type::from)
                    } else {
                        Err(Error::Message("Failed to find trait bound.".into()))
                    }
                },
                syn::Type::Infer(_) => {
                    Ok(Type::infer())
                },
                syn::Type::Macro(_) => {
                    Err(Error::Message("Macro types not supported.".into()))
                },
                syn::Type::Verbatim(_) => {
                    Err(Error::Message("Verbatim types not supported.".into()))
                },
                _ => Err(Error::Message(format!("\"{}\" not supported. Only Path, Reference and Ptr Types are currently supported", syn_type.to_token_stream()))),
            }
        }
    }
}

impl Transformer<proc_macro::TokenStream, Type> for RustTypeParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Type> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Type> for RustTypeParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Type> {
        syn::parse2::<syn::Type>(input)
            .map_err(|e| Error::Message(format!("Failed to parse type: {e}")))
            .and_then(|syn_type| self.transform(syn_type, config))
    }
}

#[cfg(test)]
mod test {
    use ligen_idl::PathSegment;

    use super::*;
    use crate::prelude::*;
    use crate::types::type_::RustTypeParser;
    use ligen::idl::Identifier;

    // FIXME: Update this tests to use the mock module.

    fn test_pairs(input: Vec<(proc_macro2::TokenStream, Type)>) {
        let v: Vec<(Type, Type)> = input
            .into_iter()
            .map(|(input, expected)| {
                (
                    RustTypeParser::new()
                        .transform(input, &Default::default())
                        .expect("Failed to parse syn::Type"),
                    expected,
                )
            })
            .collect();
        for (value, expected_value) in v {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_array() {
        test_pairs(vec![
            (quote! { [u8; 4] }, Type::array(Type::u8(), 4)),
            (quote! { [u8] }, Type::slice(Type::u8())),
        ]);
    }

    #[test]
    fn types_map() {
        test_pairs(vec![
            // (quote! { Vec<u8> }, Type::vector(Type::u8())),
        ]);
    }

    #[test]
    fn types_integer() {
        test_pairs(vec![
            (quote! { u8 }, Type::u8()),
            (quote! { u16 }, Type::u16()),
            (quote! { u32 }, Type::u32()),
            (quote! { u64 }, Type::u64()),
            (quote! { u128 }, Type::u128()),
            (quote! { usize }, Type::usize()),
            (quote! { i8 }, Type::i8()),
            (quote! { i16 }, Type::i16()),
            (quote! { i32 }, Type::i32()),
            (quote! { i64 }, Type::i64()),
            (quote! { i128 }, Type::i128()),
            (quote! { isize }, Type::isize()),
        ]);
    }

    #[test]
    fn types_float() {
        test_pairs(vec![
            (quote! { f32 }, Type::f32()),
            (quote! { f64 }, Type::f64()),
        ]);
    }

    #[test]
    fn types_boolean() -> Result<()> {
        assert_eq!(
            Type::boolean(),
            RustTypeParser::new().transform(quote! {bool}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_character() -> Result<()> {
        assert_eq!(
            Type::character(),
            RustTypeParser::new().transform(quote! {char}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            RustTypeParser::new().transform(quote! {&i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            RustTypeParser::new().transform(quote! {&mut i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            RustTypeParser::new().transform(quote! {*const i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            RustTypeParser::new().transform(quote! {*mut i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_with_generics() -> Result<()> {
        assert_eq!(
            Type::from(PathSegment::new("vec2", Type::f32())),
            RustTypeParser::new().transform(quote! {vec2<f32>}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_paren() -> Result<()> {
        assert_eq!(
            Type::i32(),
            RustTypeParser::new().transform(quote! { (i32) }, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_bare_fn() -> Result<()> {
        assert_eq!(
            Type::function(vec![Type::i32()], Type::i32()),
            RustTypeParser::new().transform(quote! { fn(i32) -> i32 }, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_never() -> Result<()> {
        assert_eq!(
            Type::void(),
            RustTypeParser::new().transform(quote! { ! }, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_infer() -> Result<()> {
        assert_eq!(
            Type::infer(),
            RustTypeParser::new().transform(quote! { _ }, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_impl_trait() -> Result<()> {
        assert_eq!(
            Type::from(Identifier::new("Trait")),
            RustTypeParser::new().transform(quote! { impl Trait }, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_dyn_trait() -> Result<()> {
        assert_eq!(
            Type::from(Identifier::new("Trait")),
            RustTypeParser::new().transform(quote! { dyn Trait }, &Default::default())?
        );
        Ok(())
    }
}
