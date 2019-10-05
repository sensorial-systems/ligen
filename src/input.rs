use quote::quote;
use proc_macro2::TokenStream;

use crate::{Method, Identifier, Type, Reference};

pub struct Input {
    pub identifier : Identifier,
    pub ty : Type
}

impl Input {
    pub fn parse(pat: &syn::PatType) -> Input {
        let identifier = match &*pat.pat {
            syn::Pat::Ident(identifier) => Some(Identifier::parse(&identifier.ident)),
            _ => None
        }.unwrap();
        let ty = Type::parse(&*pat.ty);
        Input {
            identifier,
            ty
        }
    }

    pub fn get_tokens(&self) -> (TokenStream, TokenStream) {
        let identifier = &self.identifier;
        let ty = &self.ty;
        let parameter = quote!{#identifier: #ty};
        let deref = if ty.is_atomic() { quote!{} } else { quote!{&*} };
        let arg = quote!{#deref #identifier};
        (parameter, arg)
    }
}

pub struct Inputs {
    pub self_type: Option<Type>,
    pub inputs: Vec<Input>
}

impl Inputs {
    pub fn new(self_type : Option<Type>, inputs : Vec<Input>) -> Self {
        Self {
            self_type,
            inputs
        }
    }

    pub fn parse(owner : &Type, syn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> Inputs {
        let mut inputs = Vec::new();
        let mut self_type = None;
        for input in syn_inputs {
            match input {
                syn::FnArg::Receiver(receiver) => {
                    let is_mutable = if let Some(_mutability) = receiver.mutability { true } else { false };
                    self_type = Some(Type::new(Some(Reference::new(is_mutable)), Vec::new(), Identifier::new(&owner.identifier.name)))
                },
                syn::FnArg::Typed(ty) => {
                    inputs.push(Input::parse(ty))
                }
            }
        }
        Inputs {
            self_type,
            inputs
        }
    }

    pub fn get_tokens(&self, method: &Method) -> (TokenStream, TokenStream, TokenStream) {
        let mut parameters = quote!{};
        let mut args = quote!{};
        for (i, input) in self.inputs.iter().enumerate() {
            let (parameter, arg) = input.get_tokens();
            let comma = if i > 0 { quote!{,} } else { quote!{} };
            parameters = quote!{#parameters #comma #parameter};
            args = quote!{#args #comma #arg};
        }

        let owner_type = &method.owner;

        let owner_identifier = &method.owner.identifier;
        let method_identifier = &method.identifier;

        let (parameters, method_call) = match &self.self_type {
            None => (parameters, quote!{ #owner_identifier::#method_identifier(#args) }),
            Some(_ty) => {
                let self_param = quote! { self_object : #owner_type };
                parameters = if self.inputs.len() > 0 { quote! {#self_param, #parameters} } else { self_param };
                (parameters, quote!{ (*self_object).#method_identifier(#args) })
            }
        };

        (parameters, args, method_call)
    }
}
