use ligen_ir::prelude::serde;


pub trait TypeDescriptor: schemars::JsonSchema + serde::de::DeserializeOwned {
    fn name() -> String;
    fn description() -> String;
    fn input_description() -> String;

    fn default_types() -> &'static str {
        include_str!("default_types.txt")
    }

    fn instruction() -> String {
        format!(
            "You are an universal {name} parser. You will be given a {input_description} and you will parse it into a {name} struct. Default types are:\n{default_types}",
            name = Self::name(),
            input_description = Self::input_description().to_lowercase(),
            default_types = Self::default_types()
        )
    }
}
