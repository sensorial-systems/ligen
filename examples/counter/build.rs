use ligen::prelude::*;
// use ligen_c::CGenerator;
// use ligen_cmake::{CMakeGenerator, Language};
use ligen_csharp::CSharpGenerator;

fn main() {
    if let Ok(project) = Project::read() {
        // let c_generator = CGenerator::default();
        // let cmake_generator = CMakeGenerator(Language::C);
        let csharp_generator = CSharpGenerator::default();
        // cmake_generator.generate(&project).expect("Failed to generate CMake project.");
        // c_generator.generate(&project).expect("Failed to generate C bindings");
        csharp_generator.generate(&project).expect("Failed to generate C# bindings.");
    }
}

// # 1. Create a single binding of Candle.cs
// # 2. Start marshalling module in ligen to include RString
