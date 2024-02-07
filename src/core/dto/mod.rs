use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use surrealdb::sql::Uuid;

pub mod column;
pub mod model;
pub mod table;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForeignKey {
    pub id: Option<String>,
    pub column_name: String,
    pub table_name: String,
    pub type_name: String,
}

impl ForeignKey {
    pub fn new(column_name: String, table_name: String, type_name: String) -> Self {
        Self {
            id: None,
            column_name,
            table_name,
            type_name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColumnValue {
    Bool(Option<bool>),
    Short(Option<i16>),
    Integer(Option<i32>),
    UnsignedInt(Option<u8>),
    Float(Option<f32>),
    BigFloat(Option<f64>),
    Long(Option<i64>),
    String(Option<String>),
    Uuid(Option<Uuid>),
    DateTime2(Option<NaiveDateTime>),
    DateTimeOffset(Option<NaiveDateTime>),
    Decimal(Option<Decimal>),
}

impl fmt::Display for ColumnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnValue::BigFloat(val) => format_value(f, val),
            ColumnValue::Bool(val) => format_value(f, val),
            ColumnValue::DateTime2(val) => format_value(f, val),
            ColumnValue::DateTimeOffset(val) => format_value(f, val),
            ColumnValue::Decimal(val) => format_value(f, val),
            ColumnValue::Float(val) => format_value(f, val),
            ColumnValue::Integer(val) => format_value(f, val),
            ColumnValue::Long(val) => format_value(f, val),
            ColumnValue::Short(val) => format_value(f, val),
            ColumnValue::String(val) => format_value(f, val),
            ColumnValue::UnsignedInt(val) => format_value(f, val),
            ColumnValue::Uuid(val) => format_value(f, val),
        }
    }
}

fn format_value<T>(f: &mut fmt::Formatter<'_>, value: &Option<T>) -> Result<(), std::fmt::Error>
where
    T: Display,
{
    match value {
        Some(val) => write!(f, "{}", val),
        None => write!(f, "NULL"),
    }
}
