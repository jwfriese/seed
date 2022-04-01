use std::any::Any;

use fake::locales::EN;
use fake::Fake;

use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::sql::QueryBuilder;

pub struct UsernameGenerator {
    field_name: String,
}

impl UsernameGenerator {
    pub fn new(field_name: &str) -> UsernameGenerator {
        UsernameGenerator {
            field_name: String::from(field_name),
        }
    }

    pub fn boxed_new(field_name: &str) -> Box<UsernameGenerator> {
        Box::new(UsernameGenerator::new(field_name))
    }
}

impl FieldGenerator for UsernameGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let val: String = fake::faker::internet::raw::Username(EN).fake();
        query_builder.push_string_field(&self.field_name, val.as_str());
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::fields::username::UsernameGenerator;
    use crate::generation::fields::FieldGenerator;
    use crate::sql::QueryBuilder;

    #[test]
    fn it_can_generate_usernames_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: UsernameGenerator = UsernameGenerator::new("blah");

        generator.generate_into_sql(&mut query_builder);
        let resulting_query = query_builder.finish();

        // Because this is random, this is a little tough to test. I just
        // want to ensure that the generator does not spit out blank strings.
        assert_ne!(
            resulting_query,
            String::from("INSERT INTO bananas (blah) VALUES ('')")
        );
    }
}
