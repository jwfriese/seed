use std::any::Any;

use rand::Rng;
use serde::Deserialize;

use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::generation::FieldType::Integer;
use crate::sql::QueryBuilder;

#[derive(Deserialize)]
struct IntegerOptions {
    min: Option<i64>,
    max: Option<i64>,
}

pub struct IntegerGenerator {
    field_name: String,
    options: Option<IntegerOptions>,
}

impl IntegerGenerator {
    pub fn new(field_name: &str, options: Option<&String>) -> IntegerGenerator {
        let mut field_options: Option<IntegerOptions> = None;
        if options.is_some() {
            match serde_json::from_str(options.unwrap().as_str()) {
                Ok(o) => field_options = Some(o),
                Err(json_err) => {
                    println!(
                        "Failed to deserialize into integer generator options: {}",
                        json_err
                    );
                }
            }
        }
        IntegerGenerator {
            field_name: String::from(field_name),
            options: field_options,
        }
    }

    pub fn boxed_new(field_name: &str, options: Option<&String>) -> Box<IntegerGenerator> {
        Box::new(IntegerGenerator::new(field_name, options))
    }
}

impl FieldGenerator for IntegerGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let mut min = 0;
        let mut max = 100;
        match &self.options {
            None => {}
            Some(options) => {
                match options.min {
                    None => {}
                    Some(mn) => min = mn,
                }
                match options.max {
                    None => {}
                    Some(mx) => max = mx,
                }
            }
        }
        if min > max {
            return Some(GenerationFieldError::new(
                Integer,
                format!("min ({}) cannot be greater than max ({})", min, max),
            ));
        }

        let val = rand::thread_rng().gen_range(min..max);
        query_builder.push_int_field(&self.field_name, val);
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::fields::integer::IntegerGenerator;
    use crate::generation::fields::FieldGenerator;
    use crate::sql::QueryBuilder;
    use crate::test_util::extract_value_from_sql_insert;

    #[test]
    fn it_can_generate_integers_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: IntegerGenerator = IntegerGenerator::new("blah", None);

        generator.generate_into_sql(&mut query_builder);
        let resulting_query = query_builder.finish();

        // Because this is random, this is a little tough to test. I just
        // want to ensure that the generator does not spit out a 0 when instructed
        // not to.
        assert_ne!(
            resulting_query,
            String::from("INSERT INTO bananas (blah) VALUES (0)")
        );
    }

    #[test]
    fn it_allows_assignment_of_min_and_max_integers() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: IntegerGenerator =
            IntegerGenerator::new("blah", Some(&String::from("{\"min\": -15, \"max\": 30}")));

        generator.generate_into_sql(&mut query_builder);
        let resulting_query = query_builder.finish();

        assert_ne!(
            resulting_query,
            String::from("INSERT INTO bananas (blah) VALUES (0)")
        );

        match extract_value_from_sql_insert(resulting_query.as_str()) {
            None => {
                assert!(
                    false,
                    "Expected to extract a value from a SQL statement, but found nothing"
                );
            }
            Some(val) => match val.parse::<i32>() {
                Ok(num) => {
                    assert!(num >= -15);
                    assert!(num <= 30)
                }
                Err(err) => {
                    assert!(false, "{}", err);
                }
            },
        }
    }

    #[test]
    fn it_returns_a_generation_error_when_min_is_greater_than_max() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: IntegerGenerator =
            IntegerGenerator::new("blah", Some(&String::from("{\"min\": 15, \"max\": 14}")));

        let generation_error = generator.generate_into_sql(&mut query_builder);
        assert!(generation_error.is_some());

        let error_string = format!("{}", generation_error.unwrap());

        let contains = error_string.contains("min (15) cannot be greater than max (14)");
        assert!(
            contains,
            "expected an error to get thrown containing a particular substring"
        );
    }
}
