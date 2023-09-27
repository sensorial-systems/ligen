
fn main() {
    #[cfg(feature = "bindings")]
    {
        use ligen::parsing::Parser;
        use ligen_cargo::parser::project::ProjectParser;

        let project = ProjectParser
            .parse(std::path::Path::new("D:\\dev\\sensorial\\systems\\ligen\\ecosystem\\rust\\example\\Cargo.toml"))
            .expect("Failed to parse project.");
        project.save("D:\\project.lir").expect("Failed to save project.");
    }
}
