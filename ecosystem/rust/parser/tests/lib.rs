pub mod child;

#[test]
pub fn module_file() {
    use ligen::ir::*;
    use ligen_ir::macro_attributes::Group;
    use ligen::parsing::assert::assert_eq;
    use ligen_rust_parser::module::ModuleParser;

    let module = Module {
        identifier: "lib".into(),
        functions: vec![
            Function {
                attributes: Group::from("test").into(),
                identifier: "module_file".into(),
                ..Default::default()
            }
        ],
        modules: vec![
            Module {
                identifier: "child".into(),
                functions: vec![
                    Function {
                        identifier: "child_function".into(),
                        ..Default::default()
                    }
                ],
                .. Default::default()
            }
        ],
        .. Default::default()
    };
    let project_root = project_root::get_project_root().expect("Failed to get library root.");
    let path = project_root.join(file!());
    assert_eq(ModuleParser, module, path.as_path()).unwrap()
}