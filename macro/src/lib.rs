#![feature(proc_macro_span)]

use ligen_core::proc_macro::{Context, SourceFile};
use proc_macro::TokenStream;

struct SF {
    source_file: SourceFile,
}

impl From<proc_macro::SourceFile> for SF {
    fn from(sf: proc_macro::SourceFile) -> Self {
        Self {
            source_file: SourceFile {
                is_real: sf.is_real(),
                path: sf.path(),
            },
        }
    }
}

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site().source_file();
    ligen_core::ligen(
        Context {
            source_file: SF::from(span).source_file,
        },
        args.into(),
        input.into(),
    )
    .into()
}
