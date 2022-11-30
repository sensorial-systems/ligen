// pub struct TypeImplItemMethod(pub Type, pub syn::ImplItemMethod);
//
// impl From<TypeImplItemMethod> for Function {
//     fn from(TypeImplItemMethod(owner, item_fn): TypeImplItemMethod) -> Self {
//         let syn::Signature {
//             asyncness,
//             ident,
//             inputs,
//             output,
//             ..
//         } = item_fn.sig;
//         let inputs: Vec<Parameter> = inputs
//             .clone()
//             .into_iter()
//             .map(|x| SynFnArg::from(x).try_into().expect("Failed to convert Parameter"))
//             .collect();
//         let output: Option<Type> = match output {
//             syn::ReturnType::Default => None,
//             syn::ReturnType::Type(_x, y) => {
//                 Some(Type::try_from(SynType::from(*y)).expect("Failed to convert from ReturnType::Type"))
//             }
//         };
//         let mutability = inputs
//             .get(0)
//             .map(|parameter| parameter.mutability())
//             .unwrap_or(Mutability::Constant);
//         let method = Some(Method { owner, mutability });
//         Self {
//             attributes: Attributes {
//                 attributes: item_fn
//                     .attrs
//                     .into_iter()
//                     .map(|x| SynMeta::from(x.parse_meta().expect("Failed to parse Meta")).into())
//                     .collect(),
//             },
//             method,
//             visibility: Visibility::from(SynVisibility::from(item_fn.vis)),
//             asyncness: match asyncness {
//                 Some(_x) => Some(Async),
//                 None => None,
//             },
//             path: Identifier::from(SynIdent::from(ident)).into(),
//             inputs,
//             output,
//         }
//     }
// }
