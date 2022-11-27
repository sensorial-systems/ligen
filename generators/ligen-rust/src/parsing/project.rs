
#[cfg(test)]
mod tests {
    use ligen_ir::{Module, Project, Structure, TypeDefinition};
    use ligen_ir::conventions::naming::KebabCase;
    use crate::prelude::*;
    use pretty_assertions::assert_eq;
    use ligen_utils::transformers::alias::ReplaceCrateAlias;
    use ligen_utils::transformers::path::RelativePathToAbsolutePath;
    use ligen_utils::transformers::Transformable;
    use ligen_utils::visitors::{ModuleVisitor, ProjectVisitor};

    fn mock_project(root_module: Module) -> Project {
        // FIXME: Improve the API to make this test cleaner.
        // let rust_project = Mock
        // Project::from(rust_project);
        let name = KebabCase::try_from("root").unwrap().into();
        let directory = Default::default();
        let project = Project { name, root_module, directory };
        project.transforms(&[&ReplaceCrateAlias, &RelativePathToAbsolutePath])
    }

    #[test]
    fn relative_path_to_absolute_path_imports() -> Result<()> {
        let relative_paths = quote! {
            mod root {
                mod branch {
                    use super;
                    use leaf::super::super::Root;
                    use leaf::Leaf;
                    use leaf::super::leaf::Leaf;
                    use leaf::*;
                    mod leaf {
                        use super::super::branch::Branch;
                        use super::super::Root;
                        use root::Root;
                        use root::branch::Branch;
                    }
                }
                use leaf::Leaf;
                use branch::leaf;
                use Leaf as Renamed;
                use external_crate::Something;
                fn hello(something: Something) {}
                fn get_branch() -> branch::Branch {}
                fn get_branch_ref() -> &branch::Branch {}
                fn new_leaf(branch1: &branch::Branch, leaf: &Leaf, renamed: Renamed, size: usize) -> branch::leaf::Leaf {}

                // TODO: Implement this case:
                // pub struct Object;
                // impl Object {
                //     fn hello(something: Something) {}
                //     fn get_branch() -> branch::Branch {}
                //     fn get_branch_ref() -> &branch::Branch {}
                //     fn new_leaf(branch1: &branch::Branch, leaf: &Leaf, renamed: Renamed, size: usize) -> branch::leaf::Leaf {}
                // }
            }
        };
        let absolute_paths = quote! {
            mod root {
                mod branch {
                    use root;
                    use root::Root;
                    use root::branch::leaf::Leaf;
                    use root::branch::leaf::Leaf;
                    use root::branch::leaf::*;
                    mod leaf {
                        use root::branch::Branch;
                        use root::Root;
                        use root::Root;
                        use root::branch::Branch;
                    }
                }
                use root::branch::leaf::Leaf;
                use root::branch::leaf;
                use root::branch::leaf::Leaf as Renamed;
                use external_crate::Something;
                fn hello(something: external_crate::Something) {}
                fn get_branch() -> root::branch::Branch {}
                fn get_branch_ref() -> &root::branch::Branch {}
                fn new_leaf(branch1: &root::branch::Branch, leaf: &root::branch::leaf::Leaf, renamed: root::branch::leaf::Leaf, size: usize) -> root::branch::leaf::Leaf {}
            }
        };
        let relative_paths = Module::try_from(ProcMacro2TokenStream(relative_paths))?;
        let mut absolute_paths = Module::try_from(ProcMacro2TokenStream(absolute_paths))?;
        absolute_paths.guarantee_absolute_paths();
        let project = mock_project(relative_paths);
        assert_eq!(project.root_module, absolute_paths);
        Ok(())
    }

    #[test]
    fn replace_wildcard_imports() -> Result<()> {
        let module = quote! {
            mod root {
                mod objects {
                    pub struct Object1;
                    struct Object2;
                    pub struct Object3;
                }
                pub use objects::*;
            }
        };
        let expected_module = quote! {
            mod root {
                mod objects {
                    pub struct Object1;
                    struct Object2;
                    pub struct Object3;
                }
                pub use root::objects::Object1;
                pub use root::objects::Object3;
            }
        };

        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let mut project = mock_project(module);
        project.root_module.replace_wildcard_imports();

        let mut expected_module = Module::try_from(ProcMacro2TokenStream(expected_module))?;
        expected_module.guarantee_absolute_paths();
        assert_eq!(project.root_module, expected_module);
        Ok(())
    }

    fn test_project() -> Result<ModuleVisitor> {
        let module = quote! {
            mod test_project {
                pub mod time {
                    pub mod instant {
                        use Instant as RenamedInSameModule;
                        #[ligen(opaque)]
                        pub struct Instant(std::time::Instant);
                    }
                    pub use instant::Instant;
                }
                pub use time::instant;
                pub use instant::Instant;
                pub use instant::Instant as RenamedInstant;
                pub use external_crate::internal_module;
                pub use internal_module::Something;
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let project = mock_project(module);
        Ok(ProjectVisitor::from(project).root_module_visitor())
    }

    #[test]
    fn find_absolute_path() -> Result<()> {
        let test_project = test_project()?;
        let path = Some("test_project::time::instant::Instant".into());
        assert_eq!(path, test_project.find_absolute_path(&"test_project::time::instant::Instant".into()), "Failed in absolute path case.");
        assert_eq!(path, test_project.find_absolute_path(&"self::time::instant::Instant".into()), "Failed in self path case.");
        assert_eq!(path, test_project.find_absolute_path(&"time::instant::Instant".into()), "Failed in sub-module case.");
        assert_eq!(path, test_project.find_absolute_path(&"Instant".into()), "Failed in import case.");
        assert_eq!(path, test_project.find_absolute_path(&"RenamedInstant".into()), "Failed in renamed import case.");
        assert_eq!(path, test_project.find_absolute_path(&"time::Instant".into()), "Failed in re-exported case.");
        assert_eq!(path, test_project.find_absolute_path(&"instant::Instant".into()), "Failed in re-exported submodule case.");
        assert_eq!(Some("external_crate::internal_module::Something".into()), test_project.find_absolute_path(&"Something".into()), "Failed in external crate case.");
        let time = ModuleVisitor::from(&test_project.child(test_project.current.modules[0].clone()));
        assert_eq!(path, time.find_absolute_path(&"Instant".into()), "Failed in import case in submodule.");
        assert_eq!(path, time.find_absolute_path(&"super::time::instant::Instant".into()), "Failed in super case.");
        let instant = ModuleVisitor::from(&test_project.child(time.current.modules[0].clone()));
        assert_eq!(path, instant.find_absolute_path(&"Instant".into()), "Failed to get definition path in current module.");
        assert_eq!(path, instant.find_absolute_path(&"RenamedInSameModule".into()), "Failed to renamed import in same module.");
        Ok(())
    }

    #[test]
    fn find_definition() -> Result<()> {
        let test_project = test_project()?;
        let definition = test_project.current.find_definition(&"test_project::time::instant::Instant".into());
        let expected_definition = quote! {
            #[ligen(opaque)]
            pub struct Instant(std::time::Instant);
        };
        let mut expected_definition = Structure::try_from(ProcMacro2TokenStream(expected_definition))?;
        expected_definition.path = "test_project::time::instant::Instant".into();
        let expected_definition = Some(TypeDefinition::Structure(expected_definition));
        assert_eq!(definition, expected_definition);
        Ok(())
    }
}
