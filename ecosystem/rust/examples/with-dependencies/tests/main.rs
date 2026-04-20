use ligen_idl::*;
use ligen_rust_parser::{prelude::*, RustRegistryParser};

#[tokio::test]
async fn main() -> Result<()> {
    let parser = RustRegistryParser::default();
    let registry = AsyncTransformer::transform(
        &parser,
        std::path::PathBuf::from(
            "D:\\dev\\sensorial\\systems\\ligen\\ecosystem\\rust\\examples\\with-dependencies",
        ),
        &Default::default(),
    )
    .await?;

    assert_eq!(
        registry
            .libraries
            .get(&Identifier::from("with-dependencies"))
            .unwrap()
            .root_module
            .functions[0],
        Function {
            identifier: "add".into(),
            inputs: vec![
                Parameter {
                    identifier: "left".into(),
                    type_: Type {
                        path: Path {
                            segments: vec![PathSegment {
                                identifier: Identifier { name: "u64".into() },
                                generics: Generics { types: vec![] }
                            }]
                        }
                    },
                    ..Default::default()
                },
                Parameter {
                    identifier: "right".into(),
                    type_: Type {
                        path: Path {
                            segments: vec![PathSegment {
                                identifier: Identifier { name: "u64".into() },
                                generics: Generics { types: vec![] }
                            }]
                        }
                    },
                    ..Default::default()
                }
            ],
            output: Some(Type {
                path: Path {
                    segments: vec![PathSegment {
                        identifier: Identifier { name: "u64".into() },
                        generics: Generics { types: vec![] }
                    }]
                }
            }),
            ..Default::default()
        }
    );

    assert_eq!(
        registry
            .libraries
            .get(&Identifier::from("test-fetch-from-registry"))
            .unwrap()
            .root_module
            .functions[0],
        Function {
            identifier: "sub".into(),
            inputs: vec![
                Parameter {
                    identifier: "left".into(),
                    type_: Type {
                        path: Path {
                            segments: vec![PathSegment {
                                identifier: Identifier { name: "i32".into() },
                                generics: Generics { types: vec![] }
                            }]
                        }
                    },
                    ..Default::default()
                },
                Parameter {
                    identifier: "right".into(),
                    type_: Type {
                        path: Path {
                            segments: vec![PathSegment {
                                identifier: Identifier { name: "i32".into() },
                                generics: Generics { types: vec![] }
                            }]
                        }
                    },
                    ..Default::default()
                }
            ],
            output: Some(Type {
                path: Path {
                    segments: vec![PathSegment {
                        identifier: Identifier { name: "i32".into() },
                        generics: Generics { types: vec![] }
                    }]
                }
            }),
            ..Default::default()
        }
    );

    Ok(())
}
