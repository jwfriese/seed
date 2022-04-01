use std::fmt;
use std::fmt::Formatter;

use serde::Deserialize;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Deserialize)]
pub enum FieldType {
    FirstName,
    LastName,
    FullName,
    Email,
    Username,
    Text,
    Integer,
    // Decimal,
    // Date,
    Timestamp,
    UUID,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
