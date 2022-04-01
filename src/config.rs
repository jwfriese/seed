use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Map;

use crate::generation::{
    EmailGenerator, FieldGenerator, FieldType, FirstNameGenerator, FullNameGenerator,
    IntegerGenerator, LastNameGenerator, TableGenerator, TextGenerator, TimestampGenerator,
    UsernameGenerator, UuidGenerator,
};

#[derive(Deserialize)]
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: String,
    pub tables: Vec<TableSchema>,
}

#[derive(Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub fields: Vec<FieldSchema>,
}

#[derive(Deserialize, Clone)]
pub struct FieldSchema {
    name: String,
    options: Option<Map<String, serde_json::Value>>,
    references: Option<Map<String, serde_json::Value>>,

    #[serde(alias = "type")]
    type_: FieldType,
}

impl Config {
    pub fn create_generators(self) -> HashMap<String, TableGenerator> {
        let mut generators = HashMap::new();
        for t in self.tables {
            generators.insert(
                t.name.clone(),
                TableGenerator::new(
                    t.name.clone(),
                    t.fields
                        .into_iter()
                        .map(|f| create_field_from_schema(&f))
                        .collect(),
                ),
            );
        }

        generators
    }
}

fn create_field_from_schema(f: &FieldSchema) -> Box<dyn FieldGenerator> {
    match f.type_ {
        FieldType::FullName => FullNameGenerator::boxed_new(&f.name),
        FieldType::UUID => UuidGenerator::boxed_new(&f.name),
        FieldType::FirstName => FirstNameGenerator::boxed_new(&f.name),
        FieldType::LastName => LastNameGenerator::boxed_new(&f.name),
        FieldType::Email => EmailGenerator::boxed_new(&f.name),
        FieldType::Integer => IntegerGenerator::boxed_new(&f.name),
        FieldType::Timestamp => TimestampGenerator::boxed_new(&f.name),
        FieldType::Username => UsernameGenerator::boxed_new(&f.name),
        FieldType::Text => TextGenerator::boxed_new(&f.name),
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::FieldType::FullName;
    use crate::generation::FullNameGenerator;

    use super::*;

    #[test]
    pub fn creating_generators_from_a_config() {
        let mut tables = Vec::new();
        let mut animals_table_fields: Vec<FieldSchema> = Vec::new();
        animals_table_fields.push(FieldSchema {
            type_: FullName,
            name: String::from("full_name"),
            options: Default::default(),
            references: Default::default(),
        });

        tables.push(TableSchema {
            name: "animals".to_string(),
            fields: animals_table_fields,
        });

        let config = Config {
            db_user: String::from("db_user"),
            db_password: String::from("db_password"),
            db_name: String::from("db_name"),
            db_host: String::from("db_host"),
            db_port: String::from("db_port"),
            tables,
        };

        let result = config.create_generators();
        assert_eq!(result.len(), 1);

        let get_result = result.get("animals");
        match get_result {
            None => {
                assert!(false, "Expected result to have a generator for 'animals'");
            }
            Some(animals_generator) => {
                assert_eq!(animals_generator.name, String::from("animals"));
                assert_eq!(animals_generator.fields.len(), 1);

                match animals_generator.fields[0]
                    .as_any()
                    .downcast_ref::<FullNameGenerator>()
                {
                    None => {
                        assert!(false, "Expected field result to be a FullNameGenerator");
                    }
                    Some(_) => {
                        // we're good
                    }
                }
            }
        }
    }
}
