use ligen_transformer::prelude::*;
use ligen_idl::Path;

use crate::RustIdentifierGenerator;

#[derive(Default)]
pub struct RustPathGenerator {
    identifier_generator: RustIdentifierGenerator,
}

impl Generator<&Path, syn::Path> for RustPathGenerator {
    fn generate(&self, path: &Path, _config: &Config) -> Result<syn::Path> {
        let mut path = path.clone();
        if path.segments.len() == 1 {
            let identifier = &path.segments[0].identifier.name;
            let mapped = match identifier.as_str() {
                "Boolean" => Some("bool"),
                "String" => Some("String"),
                "Character" => Some("char"),
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => Some(identifier.as_str()),
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Some(identifier.as_str()),
                "f16" | "f32" | "f64" | "f128" => Some(identifier.as_str()),
                "Vector" => Some("Vec"),
                "Dictionary" => Some("std::collections::HashMap"),
                "Opaque" => Some("serde_json::Value"),
                _ => None
            };
            if let Some(mapped) = mapped {
                let generics = path.segments[0].generics.clone();
                path = Path::from(mapped);
                if !generics.types.is_empty() {
                    path.segments.last_mut().unwrap().generics = generics;
                }
            }
        }

        let segments = path.segments.iter().map(|segment| {
            let ident = self.identifier_generator.generate(&segment.identifier, _config)?;
            let mut syn_segment = syn::PathSegment::from(ident);
            if !segment.generics.types.is_empty() {
                let mut generic_args = syn::punctuated::Punctuated::new();
                for type_ in &segment.generics.types {
                    let syn_type = crate::RustTypeGenerator::default().generate(type_, _config)?;
                    generic_args.push(syn::GenericArgument::Type(syn_type));
                }
                syn_segment.arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: generic_args,
                    gt_token: Default::default(),
                });
            }
            Ok(syn_segment)
        }).collect::<Result<Vec<_>>>()?;

        let mut syn_path = syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::new(),
        };
        for segment in segments {
            syn_path.segments.push(segment);
        }
        Ok(syn_path)
    }
}
