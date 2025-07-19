use std::rc::Rc;

use ligen_transformer::prelude::*;
use ligen_idl::Path;

pub mod path_segment;
pub use path_segment::*;

pub struct WgslPathGenerator {
    pub path_segment_generator: WgslPathSegmentGenerator,
}

impl WgslPathGenerator {
    pub fn new() -> Rc<Self> { // FIXME: If we hide this Rc as a inner, we can make this type implement Default
        Rc::new_cyclic(|weak| {
            let path_segment_generator = WgslPathSegmentGenerator::new(weak.clone());
            Self { path_segment_generator }
        })
    }
}

impl Generator<&Path, String> for WgslPathGenerator {
    fn generate(&self, path: &Path, config: &Config) -> Result<String> {
        let segments: Vec<String> = path.segments.iter().map(|segment| self.path_segment_generator.generate(segment, config)).collect::<Result<Vec<String>>>()?;
        Ok(segments.join("_"))
    }
}