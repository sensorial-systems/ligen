use ligen::ir::Module;
use crate::ast::Type;

pub fn generate_type(module: &Module, kind: &str, type_: &ligen::ir::Type) -> String {
    let generics = if let ligen::ir::Type::Composite(_, generics) = &type_ {
        if generics.types.is_empty() {
            Default::default()
        } else {
            let types = generics
                .types
                .iter()
                .map(|type_| generate_type(module, kind, type_))
                .collect::<Vec<_>>();
            format!("<{}>", types.join(", "))
        }
    } else {
        Default::default()
    };
    let type_ = Type::from(type_.clone()).to_string();
    let type_ = module
        .get_literal_from_path(format!("ligen::csharp::{}::{}::name", kind, type_))
        .map(|type_| type_.to_string())
        .unwrap_or(type_);
    format!("{}{}", type_, generics)
}