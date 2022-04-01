use std::any::Any;

use rand::Rng;

use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::sql::QueryBuilder;

pub struct IntegerGenerator {
    field_name: String,
}

impl IntegerGenerator {
    pub fn new(field_name: &str) -> IntegerGenerator {
        IntegerGenerator {
            field_name: String::from(field_name),
        }
    }

    pub fn boxed_new(field_name: &str) -> Box<IntegerGenerator> {
        Box::new(IntegerGenerator::new(field_name))
    }
}

impl FieldGenerator for IntegerGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let val = rand::thread_rng().gen_range(0..100);
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

    #[test]
    fn it_can_generate_integers_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: IntegerGenerator = IntegerGenerator::new("blah");

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
}
