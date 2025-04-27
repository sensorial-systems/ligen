use ligen_generator::prelude::*;
use ligen_ir::Path;

use crate::RustIdentifierGenerator;

#[derive(Default)]
pub struct RustPathGenerator {
    identifier_generator: RustIdentifierGenerator,
}

impl Generator<Path> for RustPathGenerator {
    type Output = syn::Path;
    fn generate(&self, path: &Path, _config: &Config) -> Result<syn::Path> {
        let segments = path.segments.iter().map(|segment| {
            let ident = self.identifier_generator.generate(&segment.identifier, _config)?;
            Ok(syn::PathSegment::from(ident))
        }).collect::<Result<Vec<_>>>()?;
        let mut path = syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::new(),
        };
        for segment in segments {
            path.segments.push_value(segment);
        }
        Ok(path)
    }
}
