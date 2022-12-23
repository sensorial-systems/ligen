use std::path::PathBuf;
use crate::prelude::*;

use syn::spanned::Spanned;
use ligen_ir::{Identifier, Path, Project};
use ligen_ir::conventions::naming::SnakeCase;
use ligen_parsing::GetPathTree;

pub struct RustProject {
    pub root_folder: PathBuf,
    pub root_module: syn::ItemMod
}

impl TryFrom<RustProject> for Project {
    type Error = Error;
    fn try_from(rust_project: RustProject) -> Result<Self> {
        let name = rust_project.get_name()?;
        let name = SnakeCase::try_from(name.as_str())?.into();
        let directory = rust_project.root_folder;
        let root_module = SynItemMod(rust_project.root_module).try_into()?;
        Ok(Self { name, directory, root_module })
    }
}

impl<'a> GetPathTree<'a> for RustProject {
    type Visitor = syn::ItemMod;
    fn get_root_visitor(&self) -> &Self::Visitor {
        &self.root_module
    }

    fn get_path(&self, module: &syn::ItemMod) -> Path {
        Identifier::from(SynIdent(module.ident.clone())).into()
    }

    fn get_children(&'a self, module: &'a syn::ItemMod) -> Vec<&'a syn::ItemMod> {
        let mut children = Vec::new();
        if let Some((_, items)) = &module.content {
            for item in items {
                if let syn::Item::Mod(module) = &item {
                    children.push(module);
                }
            }
        }
        children
    }
}

impl RustProject {
    fn get_name_from_root_folder(root_folder: &PathBuf) -> Result<String> {
        let cargo = cargo_toml::Manifest::from_path(root_folder.join("Cargo.toml")).map_err(|e| Error::Generic(Box::new(e)))?;
        let name = cargo
            .package
            .ok_or("Couldn't find the package name.")?
            .name
            .replace("-", "_");
        Ok(name)
    }

    pub fn get_name(&self) -> Result<String> {
        Self::get_name_from_root_folder(&self.root_folder)
    }

    fn parse_modules(&mut self) -> Result<()> {
        Self::parse_modules_with_current_and_parent(&mut self.root_module, None)
    }

    fn parse_modules_with_current_and_parent(current: &mut syn::ItemMod, parent: Option<&Path>) -> Result<()> {
        let identifier = Identifier::from(SynIdent(current.ident.clone()));
        let current_path = parent
            .cloned()
            .unwrap_or_default()
            .join(identifier);
        if current.content.is_none() {
            let module_path = PathBuf::from(current_path.clone());
            let content = if let Ok(content) = std::fs::read_to_string(module_path.with_extension("rs")) {
                content
            } else {
                std::fs::read_to_string(module_path.join("mod.rs"))?
            };
            let file = syn::parse_file(&content)
                .map_err(|_e| "Failed to parse file.")?;
            current.content = Some((Default::default(), file.items));
        }
        if let Some((_, items)) = &mut current.content {
            let parent = Some(&current_path);
            for item in items {
                if let syn::Item::Mod(module) = item {
                    Self::parse_modules_with_current_and_parent(module, parent)?;
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<PathBuf> for RustProject {
    type Error = Error;
    fn try_from(root_folder: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(root_folder.join("src").join("lib.rs"))?;
        let file = syn::parse_file(&content)
            .map_err(|_e| "Failed to parse lib.rs file.")?;
        let ident = syn::Ident::new(&Self::get_name_from_root_folder(&root_folder)?, file.span());
        let attrs = file.attrs;
        let content = Some((Default::default(), file.items));
        let vis = syn::Visibility::Public(syn::VisPublic { pub_token: Default::default() });
        let semi = None;
        let mod_token = Default::default();
        let root_module = syn::ItemMod { attrs, vis, mod_token, ident, semi, content };
        let mut project = Self { root_folder, root_module };
        project.parse_modules()?;
        Ok(project)
    }
}

#[cfg(test)]
impl TryFrom<TokenStream> for RustProject {
    type Error = Error;
    fn try_from(token_stream: TokenStream) -> Result<Self> {
        let root_folder = PathBuf::new();
        let root_module = syn::parse2::<syn::ItemMod>(token_stream)
            .map_err(|_e| "Failed to parse TokenStream.")?;
        let mut project = Self { root_folder, root_module };
        project.parse_modules()?;
        Ok(project)
    }
}

// impl TryFrom<RustProject> for Project {
//     type Error = Error;
//     fn try_from(project: RustProject) -> Result<Self> {
//         let name = KebabCase::try_from(project.get_name()?.as_str())?.into();
//         let directory = project.root_folder;
//         let root_module = project.root_module.into();
//         Ok(Self { name, directory, root_module })
//     }
// }

#[cfg(test)]
mod tests {
    use ligen_ir::{Constant, Function, Integer, Method, Module, Mutability, Object, Project, Structure, Type, Visibility};
    use ligen_ir::conventions::naming::KebabCase;
    use crate::prelude::*;
    use pretty_assertions::assert_eq;
    use ligen_parsing::GetPathTree;
    use ligen_utils::transformers::alias::ReplaceCrateAlias;
    use ligen_utils::transformers::path::RelativePathToAbsolutePath;
    use ligen_utils::transformers::Transformable;
    use ligen_utils::visitors::{ModuleVisitor, ProjectVisitor};
    use crate::parsing::project::RustProject;

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
        // FIXME: Remove this.
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
        // FIXME: Remove this.
        project.root_module.replace_wildcard_imports();

        let mut expected_module = Module::try_from(ProcMacro2TokenStream(expected_module))?;
        // FIXME: Remove this.
        expected_module.guarantee_absolute_paths();
        assert_eq!(project.root_module, expected_module);
        Ok(())
    }

    fn test_project() -> Result<Project> {
        let module = quote! {
            mod test_project {
                pub mod time {
                    pub mod instant {
                        use Instant as RenamedInSameModule;
                        #[ligen(opaque)]
                        pub struct Instant(std::time::Instant);

                        // FIXME: We need to test relative paths and imported paths.
                        impl test_project::time::instant::Instant {
                            const CONSTANT: i32 = 0;
                        }
                    }
                    pub use instant::Instant;

                    // FIXME: We need to test relative paths and imported paths.
                    impl test_project::time::instant::Instant {
                        fn function() {}
                    }
                }

                // FIXME: We need to test relative paths and imported paths.
                impl test_project::time::instant::Instant {
                    fn method(&self) {}
                }

                pub use time::instant;
                pub use instant::Instant;
                pub use instant::Instant as RenamedInstant;
                pub use external_crate::internal_module;
                pub use internal_module::Something;
            }
        };
        let rust_project = RustProject::try_from(module.clone())?;
        let path_tree = rust_project.get_path_tree();
        let cloned_module = ProcMacro2TokenStream(module.clone());
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let mut project = mock_project(module);
        // extract_object_implementations(&mut project, false, &cloned_module.try_into()?)?;
        Ok(project)
    }

    #[test]
    fn find_absolute_path() -> Result<()> {
        let project = test_project()?;
        let project = ProjectVisitor::from(project).root_module_visitor();
        let path = Some("test_project::time::instant::Instant".into());
        assert_eq!(path, project.find_absolute_path(&"test_project::time::instant::Instant".into()), "Failed in absolute path case.");
        assert_eq!(path, project.find_absolute_path(&"self::time::instant::Instant".into()), "Failed in self path case.");
        assert_eq!(path, project.find_absolute_path(&"time::instant::Instant".into()), "Failed in sub-module case.");
        assert_eq!(path, project.find_absolute_path(&"Instant".into()), "Failed in import case.");
        assert_eq!(path, project.find_absolute_path(&"RenamedInstant".into()), "Failed in renamed import case.");
        assert_eq!(path, project.find_absolute_path(&"time::Instant".into()), "Failed in re-exported case.");
        assert_eq!(path, project.find_absolute_path(&"instant::Instant".into()), "Failed in re-exported submodule case.");
        assert_eq!(Some("external_crate::internal_module::Something".into()), project.find_absolute_path(&"Something".into()), "Failed in external crate case.");
        let time = ModuleVisitor::from(&project.child(project.current.modules[0].clone()));
        assert_eq!(path, time.find_absolute_path(&"Instant".into()), "Failed in import case in submodule.");
        assert_eq!(path, time.find_absolute_path(&"super::time::instant::Instant".into()), "Failed in super case.");
        let instant = ModuleVisitor::from(&project.child(time.current.modules[0].clone()));
        assert_eq!(path, instant.find_absolute_path(&"Instant".into()), "Failed to get definition path in current module.");
        assert_eq!(path, instant.find_absolute_path(&"RenamedInSameModule".into()), "Failed to renamed import in same module.");
        Ok(())
    }

    #[test]
    fn find_definition() -> Result<()> {
        let project = test_project()?;
        let project = ProjectVisitor::from(project).root_module_visitor();
        let object = project.current.find_object(&"test_project::time::instant::Instant".into());
        let expected_object = quote! {
            #[ligen(opaque)]
            pub struct Instant(std::time::Instant);
        };
        let mut structure = Structure::try_from(ProcMacro2TokenStream(expected_object))?;
        structure.path = "test_project::time::instant::Instant".into();
        let expected_object = Object {
            definition: structure.into(),
            constants: vec![
                Constant {
                    path: "CONSTANT".into(),
                    type_: Integer::I32.into(),
                    literal: 0.into()
                }
            ],
            functions: vec![
                Function {
                    attributes: Default::default(),
                    inputs: Default::default(),
                    output: Default::default(),
                    asyncness: Default::default(),
                    visibility: Visibility::Private,
                    path: "function".into()
                }
            ],
            methods: vec![
                Method {
                    attributes: Default::default(),
                    inputs: Default::default(),
                    output: Default::default(),
                    asyncness: Default::default(),
                    visibility: Visibility::Private,
                    path: "method".into(),
                    mutability: Mutability::Constant,
                    owner: Type::Compound(Default::default(), Default::default())
                }
            ]
        };
        let expected_object = Some(expected_object);
        assert_eq!(object, expected_object.as_ref());
        Ok(())
    }
}
