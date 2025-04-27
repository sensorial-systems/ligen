use schemars::schema_for;
use ligen_ir::prelude::*;
pub struct JsonSchema {
    pub value: serde_json::Value,
}

impl JsonSchema {
    pub fn new<T: schemars::JsonSchema>() -> Result<Self> {
        let mut schema = serde_json::to_value(schema_for!(T))?;
        remove_property_format_value_from_json(&mut schema);
        replace_one_of_by_any_of(&mut schema);
        set_additional_properties_to_false(&mut schema);
        enforce_all_required_properties(&mut schema);

        Ok(Self { value: schema })
    }
}

pub fn set_additional_properties_to_false(object: &mut serde_json::Value) {
    match object {
        serde_json::Value::Object(object) => {
            if object.get("type") == Some(&serde_json::Value::String("object".into())) {
                object.insert("additionalProperties".into(), serde_json::Value::Bool(false));
            }
            for value in object.values_mut() {
                set_additional_properties_to_false(value);
            }
        }
        serde_json::Value::Array(array) => {
            for value in array.iter_mut() {
                set_additional_properties_to_false(value);
            }
        }
        _ => {}
    }
}

pub fn enforce_all_required_properties(object: &mut serde_json::Value) {
    match object {
        serde_json::Value::Object(object) => {
            let properties = object
                .get_mut("properties")
                .and_then(|properties| properties.as_object())
                .map(|properties|
                    properties
                        .keys()
                        .map(|key| serde_json::Value::String(key.to_string()))
                        .collect::<Vec<_>>()
                );
            if let (Some(required), Some(properties)) = (object.get_mut("required"), properties) {
                if let Some(required) = required.as_array_mut() {
                    for property in properties {
                        if !required.contains(&property) {
                            required.push(property);
                        }
                    }
                }
            }
            for value in object.values_mut() {
                enforce_all_required_properties(value);
            }
        },
        serde_json::Value::Array(array) => {
            for value in array.iter_mut() {
                enforce_all_required_properties(value);
            }
        }
        _ => {}
    }
}

pub fn replace_one_of_by_any_of(object: &mut serde_json::Value) {
    match object {
        serde_json::Value::Object(object) => {
            for key in ["oneOf", "allOf"] {
                if object.contains_key(key) {
                    if let Some(value) = object.remove(key) {
                        object.insert("anyOf".into(), value);
                    }
                }    
            }
            for value in object.values_mut() {
                replace_one_of_by_any_of(value);
            }
        }
        serde_json::Value::Array(array) => {
            for value in array.iter_mut() {
                replace_one_of_by_any_of(value);
            }
        }
        _ => {}
    }
}
pub fn remove_property_format_value_from_json(object: &mut serde_json::Value) {
    match object {
        serde_json::Value::Object(object) => {
            for key in ["minLength", "maxLength", "pattern", "format", "minimum", "maximum", "multipleOf", "patternProperties", "unevaluatedProperties", "propertyNames", "minProperties", "maxProperties", "unevaluatedItems", "contains", "minContains", "maxContains", "minItems", "maxItems", "uniqueItems"] {
                object.remove(key);
            }
            for value in object.values_mut() {
                remove_property_format_value_from_json(value);
            }
        },
        serde_json::Value::Array(array) => {
            for value in array.iter_mut() {
                remove_property_format_value_from_json(value);
            }
        },
        _ => {}
    }
}