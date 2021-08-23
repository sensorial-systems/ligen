use ligen::{ligen, ligen_dependencies};
use ligen_c::ligen_c;
use ligen_cpp::ligen_cpp;
use ligen_cmake::ligen_cmake;

pub struct Counter {
    count: u32
}

#[ligen(c, cpp)]
impl Counter {
    pub fn new(count: u32) -> Self {
        Self { count }
    }

    pub fn count(&mut self, counts: u32) {
        self.count += counts;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

pub struct Person {
    pub first_name: String,
    pub last_name: String
}

#[ligen(c, cpp)]
impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

ligen_dependencies!(c, cpp);
ligen_cmake!(c, cpp);
