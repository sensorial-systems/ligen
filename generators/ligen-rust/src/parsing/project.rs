
#[cfg(test)]
mod tests {
    use ligen_ir::{Module, Project};
    use ligen_ir::conventions::naming::{KebabCase, SnakeCase};
    use crate::prelude::*;
    use pretty_assertions::assert_eq;
    use ligen_utils::transformers::alias::ReplaceCrateAlias;
    use ligen_utils::transformers::path::RelativePathToAbsolutePath;
    use ligen_utils::transformers::Transformable;

    #[test]
    fn relative_path_to_absolute_path_imports() -> Result<()> {
        let relative_paths = quote! {
            mod root {
                mod branch {
                    use super;
                    use leaf::super::super::Root;
                    use leaf::super::Branch;
                    use leaf::Leaf;
                    use leaf::super::leaf::Leaf;
                    mod leaf {
                        use Leaf;
                        use self::Leaf;
                        use super::leaf::Leaf;
                        use super::super::branch::leaf::Leaf;
                        use super::super::branch::Branch;
                        use crate::branch::leaf::Leaf;
                        use self::super::leaf::Leaf;
                        use super::super::Root;
                        use root::Root;
                        use root::branch::Branch;
                        use root::branch::leaf::Leaf;
                    }
                }
            }
        };
        let absolute_paths = quote! {
            mod root {
                mod branch {
                    use root;
                    use root::Root;
                    use root::branch::Branch;
                    use root::branch::leaf::Leaf;
                    use root::branch::leaf::Leaf;
                    mod leaf {
                        use root::branch::leaf::Leaf;
                        use root::branch::leaf::Leaf;
                        use root::branch::leaf::Leaf;
                        use root::branch::leaf::Leaf;
                        use root::branch::Branch;
                        use root::branch::leaf::Leaf;
                        use root::branch::leaf::Leaf;
                        use root::Root;
                        use root::Root;
                        use root::branch::Branch;
                        use root::branch::leaf::Leaf;
                    }
                }
            }
        };
        let relative_paths = Module::try_from(ProcMacro2TokenStream(relative_paths))?;
        let mut absolute_paths = Module::try_from(ProcMacro2TokenStream(absolute_paths))?;
        absolute_paths.guarantee_absolute_paths();
        // FIXME: Improve the API to make this test cleaner.
        // let rust_project = Mock
        // Project::from(rust_project);
        let name = KebabCase::try_from("root")?.into();
        let root_module = relative_paths;
        let manifest_path = Default::default();
        let directory = Default::default();
        let project = Project { name, root_module, manifest_path, directory };
        let project = project.transforms(&[&ReplaceCrateAlias, &RelativePathToAbsolutePath]);
        assert_eq!(project.root_module, absolute_paths);
        Ok(())
    }

    #[test]
    fn replace_wildcard_imports() -> Result<()> {
        let module = quote! {
            mod root {
                mod object {
                    pub struct Object1;
                }
                mod objects {
                    pub struct Object2;
                    pub struct Object3;
                    struct Object4;
                    mod deeper {
                        pub struct Object5;
                        pub struct Object6;
                        struct Object7;
                    }
                    mod deeper2 {
                        pub struct Object8;
                        pub struct Object9;
                        pub struct ObjectA;
                    }
                    pub use deeper::*;
                    pub use deeper2::Object8;
                    use deeper2::Object9;
                    pub use deeper2::ObjectA as ObjectTen;
                }
                pub use object::Object1;
                pub use objects::*;
            }
        };
        let expected_module = quote! {
            mod root {
                mod object {
                    pub struct Object1;
                }
                mod objects {
                    pub struct Object2;
                    pub struct Object3;
                    struct Object4;
                    mod deeper {
                        pub struct Object5;
                        pub struct Object6;
                        struct Object7;
                    }
                    mod deeper2 {
                        pub struct Object8;
                        pub struct Object9;
                        pub struct ObjectA;
                    }
                    pub use deeper2::Object8;
                    use deeper2::Object9;
                    pub use deeper2::ObjectA as ObjectTen;
                    pub use deeper::Object5;
                    pub use deeper::Object6;
                }
                pub use object::Object1;
                pub use objects::Object2;
                pub use objects::Object3;
                pub use objects::Object8;
                pub use objects::ObjectTen;
                pub use objects::Object5;
                pub use objects::Object6;
            }
        };

        let mut module = Module::try_from(ProcMacro2TokenStream(module))?;
        module.replace_wildcard_imports();
        let project = Project {
            name: SnakeCase::try_from("project")?.into(),
            directory: Default::default(),
            manifest_path: Default::default(),
            root_module: module
        };

        let expected_module = Module::try_from(ProcMacro2TokenStream(expected_module))?;
        let expected_project = Project {
            name: SnakeCase::try_from("project")?.into(),
            directory: Default::default(),
            manifest_path: Default::default(),
            root_module: expected_module
        };

        assert_eq!(project, expected_project);
        Ok(())
    }
}
