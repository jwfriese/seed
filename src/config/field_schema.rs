use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::generation::FieldType;

#[derive(Deserialize, Clone)]
pub struct FieldSchema {
    pub name: String,

    // These are preserved as strings since fields will later attempt to
    // deserialize them into their field-specific options and references
    // types.
    #[serde(default)]
    #[serde(deserialize_with = "preserve_string")]
    pub options: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "preserve_string")]
    pub references: Option<String>,

    #[serde(rename = "type")]
    pub type_: FieldType,
}

fn preserve_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let m: Map<String, Value> = Deserialize::deserialize(deserializer)?;
    match serde_json::to_string(&m) {
        Ok(s) => Ok(Some(String::from(s))),
        Err(err) => {
            println!("error in preserve string: {}", err);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializing_a_string_into_field_schema() {
        let input_str = "{
          \"name\": \"age\",
          \"type\": \"Integer\",
          \"options\": {
            \"max\": 75,
            \"min\": 18
          }}";
        let result = serde_json::from_str(input_str);
        match result {
            Ok(s) => {
                let field_schema: FieldSchema = s;
                assert_eq!(field_schema.name, "age");
                assert_eq!(field_schema.type_, FieldType::Integer);

                match field_schema.options {
                    None => assert!(false, "Failed to deserialize the options field"),
                    Some(o) => {
                        assert_eq!(o, String::from("{\"max\":75,\"min\":18}"));
                    }
                }
            }
            Err(err) => {
                assert!(false, "Expected to deserialize a field schema: {}", err);
            }
        }
    }
}
