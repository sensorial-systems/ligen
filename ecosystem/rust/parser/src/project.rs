use std::path::PathBuf;
use crate::prelude::*;

use syn::spanned::Spanned;
use ligen_ir::{Identifier, Module, Path, Project};
use ligen_ir::conventions::naming::SnakeCase;
use ligen_parsing::{GetPathTree, Context, ParseFrom};

pub struct RustProject {
    pub root_folder: PathBuf,
    pub root_module: syn::ItemMod
}

impl ParseFrom<RustProject> for Project {
    fn parse_from(context: &Context<'_>, rust_project: RustProject) -> Result<Self> {
        let name = rust_project.get_name().unwrap_or_default();
        let name = SnakeCase::try_from(name.as_str())?.into();
        let directory = rust_project.root_folder;
        let mut root_module = Module::parse_from(context, SynItemMod(rust_project.root_module))?;
        module_full_path(&mut root_module, context);
        object_full_path(&mut root_module);
        import_full_path(&mut root_module, context);
        Ok(Self { name, directory, root_module })
    }
}

fn import_full_path(module: &mut Module, context: &Context) {
    for import in &mut module.imports {
        if let Some(path) = context.path_tree.find_from_relative_path(import.path.clone()) { // for module import
            import.path = path.data.clone();
        } else if let Some(path) = context.path_tree.find_from_relative_path(import.path.clone().without_last()) { // for object import (we don't have the objects in the path tree)
            import.path = path.data.clone().join(import.path.last());
        }
    }
    for module in &mut module.modules {
        import_full_path(module, &context.switch_to(module.path.last()));
    }
}

fn object_full_path(module: &mut Module) {
    for object in &mut module.objects {
        object.path = module.path.clone().join(object.path.clone());
    }
    for module in &mut module.modules {
        object_full_path(module)
    }
}

fn module_full_path(module: &mut Module, context: &Context) {
    module.path = context.path.clone();
    for module in &mut module.modules {
        module_full_path(module, &context.switch_to(module.path.last()));
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

    fn get_children<'b>(&'b self, module: &'b syn::ItemMod) -> Vec<&'b Self::Visitor> {
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
        let cargo = cargo_toml::Manifest::from_path(root_folder.join("../../../../Cargo.toml")).map_err(|e| Error::Generic(Box::new(e)))?;
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
                std::fs::read_to_string(module_path.join("string_editable_field"))?
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

impl TryFrom<ProcMacro2TokenStream> for RustProject {
    type Error = Error;
    fn try_from(ProcMacro2TokenStream(value): ProcMacro2TokenStream) -> Result<Self> {
        value.try_into()
    }
}

impl TryFrom<TokenStream> for RustProject {
    type Error = Error;
    fn try_from(token_stream: TokenStream) -> Result<Self> {
        let root_folder = PathBuf::new();
        let root_module = syn::parse2::<syn::ItemMod>(token_stream)
            .map_err(|e| format!("Failed to parse TokenStream ({:?}).", e))?;
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
    use ligen_ir::{Constant, Function, Import, Integer, Method, Module, Mutability, Object, Project, Structure, Type, Visibility};
    use ligen_parsing::{Context, GetPathTree, ParseFrom};
    use crate::prelude::*;
    use pretty_assertions::assert_eq;
    use ligen_utils::visitors::{ModuleVisitor, ProjectVisitor};
    use crate::project::RustProject;

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
        let rust_project = RustProject::try_from(ProcMacro2TokenStream(relative_paths))?;
        let path_tree = rust_project.get_path_tree();
        let context = Context::from(&path_tree);
        let project = Project::parse_from(&context, rust_project)?;

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
        let rust_project = RustProject::try_from(absolute_paths.clone())?;
        let path_tree = rust_project.get_path_tree();
        let context = Context::from(&path_tree);
        let mut absolute_paths = Module::parse_from(&context, ProcMacro2TokenStream(absolute_paths))?;
        // FIXME: Remove this.
        absolute_paths.guarantee_absolute_paths();
        assert_eq!(project.root_module, absolute_paths);
        Ok(())
    }

    #[test]
    fn guaranteed_absolute_paths() -> Result<()> {
        let module = quote! {
            mod root {
                struct Root;
                mod branch {
                    struct Branch;
                    mod leaf {
                        struct Leaf;
                    }
                    use leaf::Leaf;
                }
                use branch::leaf;
                // use leaf::Leaf;
            }
        };
        let rust_project = RustProject::try_from(module)?;
        let path_tree = rust_project.get_path_tree();
        let context = Context::from(&path_tree);
        let project = Project::parse_from(&context, rust_project)?;
        let expected_module = Module {
            path: "root".into(),
            objects: vec![ Object { path: "root::Root".into(), ..Default::default() } ],
            imports: vec![ Import { path: "root::branch::leaf".into(), ..Default::default() }],
            modules: vec![
                Module {
                    path: "root::branch".into(),
                    objects: vec![ Object { path: "root::branch::Branch".into(), ..Default::default() }.into() ],
                    imports: vec![ Import { path: "root::branch::leaf::Leaf".into(), ..Default::default() } ],
                    modules: vec![
                        Module {
                            path: "root::branch::leaf".into(),
                            objects: vec![ Object { path: "root::branch::leaf::Leaf".into(), ..Default::default() }.into() ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        };
        assert_eq!(project.root_module, expected_module);
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

        let rust_project = RustProject::try_from(ProcMacro2TokenStream(module))?;
        // FIXME: Ideally do it all in a single line. Parse::parse(ParseFrom<T>) -> U
        let path_tree = rust_project.get_path_tree();
        let context = Context::from(&path_tree);
        let mut project = Project::parse_from(&context, rust_project)?;
        // FIXME: Remove this.
        project.root_module.guarantee_absolute_paths();
        project.root_module.replace_wildcard_imports();

        let mut expected_module = Module::parse_from(&context, ProcMacro2TokenStream(expected_module))?;
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
        let context = Context::from(&path_tree);
        let project = Project::parse_from(&context, rust_project)?;
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
        let structure = Structure::try_from(ProcMacro2TokenStream(expected_object))?;
        let expected_object = Object {
            attributes: Default::default(),
            visibility: Visibility::Public,
            path: "test_project::time::instant::Instant".into(),
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
                    synchrony: Default::default(),
                    visibility: Visibility::Private,
                    path: "function".into()
                }
            ],
            methods: vec![
                Method {
                    attributes: Default::default(),
                    inputs: Default::default(),
                    output: Default::default(),
                    synchrony: Default::default(),
                    visibility: Visibility::Private,
                    path: "method".into(),
                    mutability: Mutability::Constant,
                    owner: Type::Composite(Default::default(), Default::default())
                }
            ]
        };
        let expected_object = Some(expected_object);
        assert_eq!(object, expected_object.as_ref());
        Ok(())
    }
}
