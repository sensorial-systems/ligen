use quote::quote;
use proc_macro2::TokenStream;

use crate::{Method, Identifier, Type, TypeModifier, Reference};

pub struct Input {
    pub identifier : Identifier,
    pub typ : Type
}

impl Input {
    pub fn parse(pat: &syn::PatType) -> Input {
        let identifier = match &*pat.pat {
            syn::Pat::Ident(identifier) => Some(Identifier::parse(&identifier.ident)),
            _ => None
        }.unwrap();
        let typ = Type::parse(&*pat.ty);
        Input {
            identifier,
            typ
        }
    }

    pub fn get_tokens(&self) -> (TokenStream, TokenStream) {
        let identifier = &self.identifier;
        let typ = &self.typ;
        let parameter = quote!{#identifier: #typ};
        let arg = if typ.is_atomic() {
            quote!{#identifier}
        } else {
            match &typ.modifier {
                TypeModifier::Reference(_) => quote!{&* #identifier},
                TypeModifier::Pointer(_) => quote!{#identifier},
                TypeModifier::None => quote!{*Box::from_raw(#identifier)}
            }
        };
        (parameter, arg)
    }
}

pub struct Inputs {
    pub self_type: Option<Type>,
    pub self_type_mutability : bool,
    pub inputs: Vec<Input>
}

impl Inputs {
    pub fn new(self_type : Option<Type>, self_type_mutability: bool, inputs : Vec<Input>) -> Self {
        Self {
            self_type,
            self_type_mutability,
            inputs
        }
    }

    pub fn parse(owner : &Type, syn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> Inputs {
        let mut inputs = Vec::new();
        let mut self_type = None;
        let mut self_type_mutability = false;
        for input in syn_inputs {
            match input {
                syn::FnArg::Receiver(receiver) => {
                    self_type_mutability = if let Some(_mutability) = receiver.mutability { true } else { false };
                    let modifier = if let Some(_) = receiver.reference {
                        TypeModifier::Reference(Reference::new(self_type_mutability))
                    } else {
                        TypeModifier::None
                    };
                    self_type = Some(Type::new(modifier, Vec::new(), Identifier::new(&owner.identifier.name)))
                },
                syn::FnArg::Typed(ty) => {
                    inputs.push(Input::parse(ty))
                }
            }
        }
        Inputs::new(self_type, self_type_mutability, inputs)
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
            Some(typ) => {
                let self_param = quote! { self_object : #owner_type };
                parameters = if self.inputs.len() > 0 { quote! {#self_param, #parameters} } else { self_param };
                match &typ.modifier {
                    TypeModifier::Pointer(_) => (parameters, quote!{ (*self_object).#method_identifier(#args) }),
                    TypeModifier::Reference(_) => (parameters, quote!{ (*self_object).#method_identifier(#args) }),
                    TypeModifier::None => (parameters, quote!{ (Box::from_raw(self_object)).#method_identifier(#args) })
                }
            }
        };

        (parameters, args, method_call)
    }
}
