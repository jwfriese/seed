use std::any::Any;

use fake::locales::EN;
use fake::Fake;

use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::sql::QueryBuilder;

pub struct TimestampGenerator {
    field_name: String,
}

impl TimestampGenerator {
    pub fn new(field_name: &str) -> TimestampGenerator {
        TimestampGenerator {
            field_name: String::from(field_name),
        }
    }

    pub fn boxed_new(field_name: &str) -> Box<TimestampGenerator> {
        Box::new(TimestampGenerator::new(field_name))
    }
}

impl FieldGenerator for TimestampGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let val: String = fake::faker::chrono::raw::DateTime(EN).fake();
        query_builder.push_string_field(&self.field_name, val.as_str());
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::fields::timestamp::TimestampGenerator;
    use crate::generation::fields::FieldGenerator;
    use crate::sql::QueryBuilder;

    #[test]
    fn it_can_generate_emails_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: TimestampGenerator = TimestampGenerator::new("blah");

        generator.generate_into_sql(&mut query_builder);
        let resulting_query = query_builder.finish();

        // Because this is random, this is a little tough to test. I just
        // want to ensure that the generator does not spit out a blank timestamp.
        assert_ne!(
            resulting_query,
            String::from("INSERT INTO bananas (blah) VALUES ('')")
        );
    }
}
