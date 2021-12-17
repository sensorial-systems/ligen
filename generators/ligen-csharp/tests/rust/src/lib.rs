pub struct Test {}

impl Test {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn get_name() -> String {
        "Name".into()
    }
}

pub mod string;