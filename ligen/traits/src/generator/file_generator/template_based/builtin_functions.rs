// TODO: Document built-in functions.

use ligen_ir::Path;
use crate::generator::file_generator::Inputs;


pub fn json(inputs: &Inputs) -> String {
    let parameter = inputs.get(0).and_then(|input| serde_json::to_string(&input).ok());
    parameter.unwrap_or("<ligen:json error>".into())
}

pub fn join_path(inputs: &Inputs) -> String {
    let separator = inputs
        .get(0)
        .and_then(|input| serde_json::from_value::<String>(input).ok());
    let path = inputs
        .get(1)
        .and_then(|input| serde_json::from_value::<Path>(input).ok());
    if let (Some(separator), Some(path)) = (separator, path) {
        path.to_string(&separator)
    } else {
        format!("<ligen:join_path error>")
    }
}

pub fn name_from_path(inputs: &Inputs) -> String {
    let path = inputs
        .get(0)
        .and_then(|input| serde_json::from_value::<Path>(input).ok());
    if let Some(path) = path {
        let content = path.last();
        content.name
    } else {
        format!("<ligen:name_from_path error>")
    }
}
