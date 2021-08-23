use ligen::prelude::*;
use ligen_c::CGenerator;
use ligen_cmake::{CMakeGenerator, Language};

fn main() {
    if let Ok(project) = Project::read() {
        let c_generator = CGenerator::default();
        let cmake_generator = CMakeGenerator::new(Language::C);
        cmake_generator.generate(&project).expect("Couldn't generate CMake project.");
        c_generator.generate(&project).expect("Couldn't generate C bindings");
    }
}
