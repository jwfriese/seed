use fake::Fake;
use std::any::Any;

use crate::generation::error::GenerationFieldError;
use crate::generation::fields::FieldGenerator;
use crate::sql::QueryBuilder;

pub struct TextGenerator {
    field_name: String,
}

impl TextGenerator {
    pub fn new(field_name: &str) -> TextGenerator {
        TextGenerator {
            field_name: String::from(field_name),
        }
    }

    pub fn boxed_new(field_name: &str) -> Box<TextGenerator> {
        Box::new(TextGenerator::new(field_name))
    }
}

impl FieldGenerator for TextGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError> {
        let val: String = fake::faker::lorem::en::Paragraph(2..4).fake();
        query_builder.push_string_field(&self.field_name, val.as_str());
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::fields::text::TextGenerator;
    use crate::generation::fields::FieldGenerator;
    use crate::sql::QueryBuilder;

    #[test]
    fn it_can_generate_text_into_sql_statements() {
        let mut query_builder = QueryBuilder::new("bananas");
        let generator: TextGenerator = TextGenerator::new("blah");

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
