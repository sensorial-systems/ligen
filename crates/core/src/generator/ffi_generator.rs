//! FFI generator module.

use crate::prelude::*;

use crate::generator::{Context, ImplementationVisitor, FunctionVisitor};
use crate::ir::{Parameter, Type, Identifier, Visibility, ImplementationItem};

/// FFI generator.
pub trait FFIGenerator {
    /// Generate FFI.
    fn generate_ffi(&self, context: &Context, visitor: Option<&ImplementationVisitor>) -> TokenStream;
}

/// A generic FFI generator which can be used for most languages.
pub trait GenericFFIGenerator {
    /// Generate the function parameters.
    fn generate_parameters(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let type_ = Self::to_marshal_parameter(&parameter.type_);
                let identifier = &parameter.identifier;
                tokens.append_all(quote! {#identifier: #type_,});
                tokens
            })
    }

    /// Generate the function call arguments and its conversions.
    fn generate_arguments(_context: &Context, inputs: &Vec<Parameter>) -> TokenStream {
        inputs
            .iter()
            .fold(TokenStream::new(), |mut tokens, parameter| {
                let identifier = &parameter.identifier;
                tokens.append_all(quote! {#identifier.into(),});
                tokens
            })
    }

    /// Marshal type.
    fn to_marshal_output(type_: &Type) -> TokenStream {
        match type_ {
            Type::Compound(path) => match path.segments.last().unwrap().name.as_str() {
                "String" => quote! { *mut crate::ffi::RString },
                _ => quote! { *mut #type_ },
            },
            _ => quote! { #type_ },
        }
    }

    /// Marshal type.
    fn to_marshal_parameter(type_: &Type) -> TokenStream {
        match type_ {
            Type::Compound(path) => match path.segments.last().unwrap().name.as_str() {
                "String" => quote! { crate::ffi::CChar },
                _ => quote! { *mut #type_ },
            },
            _ => quote! { #type_ },
        }
    }

    /// Generate the function output.
    fn generate_output(_context: &Context, output: &Option<Type>) -> TokenStream {
        match output {
            Some(type_) => Self::to_marshal_output(type_),
            _ => quote! {()},
        }
    }

    /// Generate the function
    fn generate_function_signature(
        context: &Context,
        visitor: &FunctionVisitor,
    ) -> TokenStream {
        let implementation = &visitor.parent.current;
        let function = &visitor.current;
        let parameters = Self::generate_parameters(context, &function.inputs);
        let output = Self::generate_output(context, &function.output);
        let function_name = format!("{}_{}", implementation.self_.name, function.identifier.name);
        let function_identifier = Identifier::new(&function_name);
        quote! {
            #[no_mangle]
            pub extern fn #function_identifier(#parameters) -> #output
        }
    }

    /// Generate the function
    fn generate_function_block(
        context: &Context,
        visitor: &FunctionVisitor,
    ) -> TokenStream {
        let method = &visitor.current;
        let implementation = &visitor.parent.current;
        let arguments = Self::generate_arguments(context, &method.inputs);
        let self_identifier = &implementation.self_;
        let method_identifier = &method.identifier;
        let result = if let Some(Type::Compound(_identifier)) = method.output.as_ref() {
            quote! {
                Box::into_raw(Box::new(result.into()))
            }
        } else {
            quote! {result}
        };
        quote! {
            {
                let result = #self_identifier::#method_identifier(#arguments);
                #result
            }
        }
    }

    /// Generate an extern function for an implementation method.
    fn generate_function(
        context: &Context,
        visitor: &FunctionVisitor,
    ) -> TokenStream {
        if let Visibility::Public = visitor.current.visibility {
            let function_signature = Self::generate_function_signature(context, visitor);
            let method_block = Self::generate_function_block(context, visitor);
            quote! { #function_signature #method_block }
        } else {
            quote! {}
        }
    }

    /// Generate drop extern.
    fn generate_drop(visitor: &ImplementationVisitor) -> TokenStream {
        let object_name = &visitor.current.self_;
        let drop_name = Identifier::new(format!("{}_drop", object_name.name).as_str());
        quote! {
            #[no_mangle]
            pub unsafe extern fn #drop_name(object: *mut #object_name) {
                Box::from_raw(object);
            }
        }
    }

    /// Generate externs for Constants and Methods.
    fn generate(context: &Context, implementation: &ImplementationVisitor) -> TokenStream {
        let mut tokens =
            implementation
                .current
                .items
                .iter()
                .fold(TokenStream::new(), |mut tokens, item| {
                    match item {
                        ImplementationItem::Constant(_) => unimplemented!("Constants aren't implemented yet."),
                        ImplementationItem::Method(method) => tokens.append_all(Self::generate_function(context, &implementation.child(method.clone()))),
                    }
                    tokens
                });
        tokens.append_all(Self::generate_drop(&implementation));
        tokens
    }
}

impl<T: GenericFFIGenerator> FFIGenerator for T {
    fn generate_ffi(&self, context: &Context, implementation: Option<&ImplementationVisitor>) -> TokenStream {
        implementation
            .map(|implementation| Self::generate(context, implementation))
            .unwrap_or_else(|| TokenStream::new())
    }
}
