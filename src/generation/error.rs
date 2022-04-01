use crate::generation::fields::field_type::FieldType;
use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

#[derive(Deserialize)]
pub struct GenerationTableError {
    name: String,
    details: String,
}

impl GenerationTableError {
    pub fn new(table_name: &str, details: &str) -> GenerationTableError {
        GenerationTableError {
            name: String::from(table_name),
            details: String::from(details),
        }
    }
}

impl fmt::Debug for GenerationTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GenerationTableError")
            .field("name", &self.name)
            .field("details", &self.details)
            .finish()
    }
}

impl fmt::Display for GenerationTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(table_name: {}, details: {})", self.name, self.details)
    }
}

#[derive(Deserialize)]
pub struct GenerationFieldError {
    field_type: FieldType,
    details: String,
}

impl GenerationFieldError {
    pub fn new(field_type: FieldType, details: String) -> GenerationFieldError {
        GenerationFieldError {
            field_type,
            details,
        }
    }
}

impl fmt::Debug for GenerationFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GenerationFieldError")
            .field("field_type", &self.field_type)
            .field("details", &self.details)
            .finish()
    }
}

impl fmt::Display for GenerationFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(field_type: {}, details: {})",
            self.field_type, self.details
        )
    }
}
