#![feature(proc_macro_span)]

use ligen_core::proc_macro::{Context, SourceFile, Arguments};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    let source_file = proc_macro::Span::call_site().source_file();
    let source_file = SourceFile {
        is_real: source_file.is_real(),
        path: source_file.path()
    };
    let arguments = Arguments::from_env().expect("Couldn't build Arguments from enviroment variables.");
    let context = Context { source_file, arguments };
    ligen_core::ligen(context, args.into(), input.into()).into()
}
