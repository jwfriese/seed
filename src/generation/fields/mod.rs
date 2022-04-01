use crate::generation::error::GenerationFieldError;
use crate::sql::QueryBuilder;
use std::any::Any;

pub mod email;
pub mod field_type;
pub mod first_name;
pub mod full_name;
pub mod integer;
pub mod last_name;
pub mod text;
pub mod timestamp;
pub mod username;
pub mod uuid;

pub trait FieldGenerator {
    fn generate_into_sql(&self, query_builder: &mut QueryBuilder) -> Option<GenerationFieldError>;
    fn as_any(&self) -> &dyn Any;
}
