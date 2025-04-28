use ligen_ir::prelude::serde;


pub trait TypeDescriptor: schemars::JsonSchema + serde::de::DeserializeOwned {
    fn name() -> String;
    fn description() -> String;
    fn input_description() -> String;

    fn default_types() -> &'static str {
        include_str!("default_types.txt")
    }
}
