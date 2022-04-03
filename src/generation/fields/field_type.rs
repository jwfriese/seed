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
        match self {
            FieldType::FirstName => {
                write!(f, "FirstName")
            }
            FieldType::LastName => {
                write!(f, "LastName")
            }
            FieldType::FullName => {
                write!(f, "FullName")
            }
            FieldType::Email => {
                write!(f, "Email")
            }
            FieldType::Username => {
                write!(f, "Username")
            }
            FieldType::Text => {
                write!(f, "Text")
            }
            FieldType::Integer => {
                write!(f, "Integer")
            }
            FieldType::Timestamp => {
                write!(f, "Timestamp")
            }
            FieldType::UUID => {
                write!(f, "UUID")
            }
        }
    }
}
