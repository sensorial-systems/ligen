use ligen::prelude::*;
use ligen_csharp::CSharpGenerator;

fn main() {
    match Project::current() {
        Ok(project) => {
            let csharp_generator = CSharpGenerator::default();
            csharp_generator.generate(&project).expect("Failed to generate C# bindings.");
        },
        Err(e) => println!("e: {:#?}", e)
    }
}

