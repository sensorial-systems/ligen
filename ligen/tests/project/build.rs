fn main() {
    #[cfg(feature = "bindings")]
        {
            use ligen::prelude::*;
            // use ligen_csharp::CSharpGenerator;
            use ligen_cargo::{CargoProject, CargoGenerator};
            use ligen_rust::RustGenerator;

            match CargoProject::current().and_then(Project::try_from) {
                Ok(project) => {
                    CargoGenerator::default().generate(&project).expect("Failed to generate Cargo interface.");
                    RustGenerator::default().generate(&project).expect("Failed to generate Rust interface.");
                    // CSharpGenerator::default().generate(&project).expect("Failed to generate C# interface.");
                    // CargoBuilder.build(&project, BuildProfile::Release).expect("Failed to build Cargo project.");
                },
                Err(_) => ()
            }
        }
}