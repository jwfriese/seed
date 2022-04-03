use crate::sql::value::Value;
use crate::sql::{BoolValue, IntValue, StringValue};

struct Field {
    name: String,
    value: Box<dyn Value>,
}

pub struct QueryBuilder {
    table: String,
    fields: Vec<Field>,
}

impl QueryBuilder {
    pub fn new(table: &str) -> QueryBuilder {
        QueryBuilder {
            table: String::from(table),
            fields: vec![],
        }
    }

    pub fn push_string_field(&mut self, name: &str, value: &str) -> &mut QueryBuilder {
        self.fields.push(Field {
            name: String::from(name),
            value: Box::new(StringValue::new(String::from(value))),
        });
        self
    }

    pub fn push_int_field(&mut self, name: &str, value: i64) -> &mut QueryBuilder {
        self.fields.push(Field {
            name: String::from(name),
            value: Box::new(IntValue::new(value)),
        });
        self
    }

    pub fn push_bool_field(&mut self, name: &str, value: bool) -> &mut QueryBuilder {
        self.fields.push(Field {
            name: String::from(name),
            value: Box::new(BoolValue::new(value)),
        });
        self
    }

    pub fn finish(&self) -> String {
        let field_names = self
            .fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        let values = &[
            "(",
            self.fields
                .iter()
                .map(|f| f.value.to_sql_element())
                .collect::<Vec<String>>()
                .join(", ")
                .as_str(),
            ")",
        ]
        .join("");

        let fields_concat = &["(", field_names.as_str(), ") "].join("");

        [
            "INSERT INTO ",
            &self.table,
            " ",
            fields_concat,
            "VALUES ",
            values,
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it() {
        let mut builder = QueryBuilder::new("bananas");
        let result = builder
            .push_int_field("int_field", 10)
            .push_string_field("string_field", "this is a value")
            .push_bool_field("bool_field", true)
            .finish();

        assert_eq!(
            result,
            [
                r#"INSERT INTO bananas"#,
                r#"(int_field, string_field, bool_field)"#,
                r#"VALUES (10, 'this is a value', TRUE)"#,
            ]
            .join(" ")
        )
    }
}
