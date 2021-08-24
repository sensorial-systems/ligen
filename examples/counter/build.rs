use ligen::prelude::*;
// use ligen_c::CGenerator;
// use ligen_cmake::{CMakeGenerator, Language};
use ligen_csharp::CSharpGenerator;

fn main() {
    if let Ok(project) = Project::read() {
        // let c_generator = CGenerator::default();
        // let cmake_generator = CMakeGenerator::new(Language::C);
        // cmake_generator.generate(&project).expect("Couldn't generate CMake project.");
        // c_generator.generate(&project).expect("Couldn't generate C bindings");
        let csharp_generator = CSharpGenerator::default();
        csharp_generator.generate(&project).expect("Couldnt generate C# bindings.");
    }
}

// # 1. Create a single binding of Candle.cs
// # 2. Start marshalling module in ligen to include RString
