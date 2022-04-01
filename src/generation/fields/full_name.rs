use std::any::Any;

use fake::Fake;
use fake::locales::EN;
use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;

use crate::sql::QueryBuilder;

pub struct FullNameGenerator {
    name: String,
}

impl FullNameGenerator {
    pub fn new(name: &str) -> FullNameGenerator {
        FullNameGenerator {
            name: String::from(name)
        }
    }

    pub fn boxed_new(name: &str) -> Box<FullNameGenerator> {
        Box::new(FullNameGenerator::new(name))
    }
}

impl FieldGenerator for FullNameGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let val: String = fake::faker::name::raw::Name(EN).fake();
        query_builder.push_string_field(&self.name, val.as_str());
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::fields::FieldGenerator;
    use crate::generation::FullNameGenerator;
    use crate::sql::QueryBuilder;

    #[test]
    fn it_can_generate_full_names_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: FullNameGenerator = FullNameGenerator::new("blah");

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
