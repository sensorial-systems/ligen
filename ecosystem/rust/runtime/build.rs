
fn main() {
    #[cfg(feature = "bindings")]
        {
            use ligen::prelude::*;
            use ligen_cargo::{CargoLibrary, CargoGenerator, CargoBuilder};
            use ligen_rust::RustGenerator;
            use ligen_c::CGenerator;
            use ligen_cmake::{CMakeGenerator, Language};
            use ligen::traits::build::BuildSystem;

            match CargoLibrary::current().and_then(Library::try_from) {
                Ok(library) => {
                    CargoGenerator::default().generate(&library).expect("Failed to generate Cargo interface.");
                    RustGenerator::default().generate(&library).expect("Failed to generate Rust interface.");
                    CGenerator::default().generate(&library).expect("Failed to generate C interface.");
                    CMakeGenerator(Language::C).generate(&library).expect("Failed to generate CMake library.");
                    CargoBuilder.build(&library).expect("Failed to build Cargo library.");
                },
                Err(_) => ()
            }
        }
}
