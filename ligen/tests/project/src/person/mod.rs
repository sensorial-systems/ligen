pub mod sub_counter;

pub struct Person {
    pub first_name: String,
    pub last_name: String
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
