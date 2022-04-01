use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::sql::{Statement, QueryBuilder};

pub struct TableGenerator {
    pub name: String,
    pub fields: Vec<Box<dyn FieldGenerator>>,
}

impl TableGenerator {
    pub fn new(name: String, fields: Vec<Box<dyn FieldGenerator>>) -> TableGenerator {
        TableGenerator {
            name,
            fields,
        }
    }

    pub fn generate_statement(&self) -> Result<Statement, GenerationFieldError> {
        let mut builder = QueryBuilder::new(self.name.as_str());
        for f in &self.fields {
            let field_generation_error = f.generate_into_sql(&mut builder);
            match field_generation_error {
                None => {}
                Some(err) => {
                    return Err(err);
                }
            }
        }

        Ok(Statement::new(builder.finish().as_str()))
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn deserializing_a_table_generator_works() {
    //     let input_datetime = NaiveDateTime::new(
    //         NaiveDate::from_ymd(2035, 6, 5),
    //         NaiveTime::from_hms(6, 5, 0),
    //     );
    //
    //     let input = QueryDateTimeString {
    //         parsed_datetime: input_datetime,
    //     };
    //     assert_tokens(
    //         &input,
    //         &[
    //             Token::Struct {
    //                 name: "QueryDateTimeString",
    //                 len: 1,
    //             },
    //             Token::Str("parsed_datetime"),
    //             Token::Str("2035-06-05 06-05-00"),
    //             Token::StructEnd,
    //         ],
    //     );
    // }
}
