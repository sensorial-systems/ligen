
fn main() {
    #[cfg(feature = "bindings")]
    {
        use ligen::parsing::parser::Parser;
        use ligen_cargo::parser::project::ProjectParser;

        // FIXME: Hardcoded absolute path.
        let project = ProjectParser
            .parse(std::path::Path::new("D:\\dev\\sensorial\\systems\\ligen\\ecosystem\\rust\\example\\Cargo.toml"))
            .expect("Failed to parse project.");
        project.save("D:\\project.lir").expect("Failed to save project.");
    }
}
