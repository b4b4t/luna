use std::str::FromStr;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use tiberius::{ColumnData, FromSql};

use crate::core::dto::ColumnValue;

// Convert a ColumnData to ColumnValue in order to save it in the local SurrealDb
pub fn to_column_value(column_data: ColumnData<'static>) -> anyhow::Result<ColumnValue> {
    let data = column_data.clone();
    match data {
        ColumnData::Bit(value) => Ok(ColumnValue::Bool(value)),
        ColumnData::F32(value) => Ok(ColumnValue::Float(value)),
        ColumnData::F64(value) => Ok(ColumnValue::BigFloat(value)),
        ColumnData::I16(value) => Ok(ColumnValue::Short(value)),
        ColumnData::I32(value) => Ok(ColumnValue::Integer(value)),
        ColumnData::I64(value) => Ok(ColumnValue::Long(value)),
        ColumnData::U8(value) => Ok(ColumnValue::UnsignedInt(value)),
        ColumnData::String(value) => {
            let string_value = match value {
                Some(cow_value) => Some(cow_value.to_string()),
                None => None,
            };

            Ok(ColumnValue::String(string_value))
        }
        ColumnData::Binary(_) => Err(anyhow::anyhow!("Cannot convert binary value")),
        ColumnData::DateTime2(value) => {
            let datetime = match value {
                Some(_) => NaiveDateTime::from_sql(&column_data)
                    .expect("Cannot convert Datetime2 to NaiveDateTime"),
                None => None,
            };
            Ok(ColumnValue::DateTime2(datetime))
        }
        ColumnData::DateTimeOffset(value) => {
            let datetime = match value {
                Some(_) => NaiveDateTime::from_sql(&column_data)
                    .expect("Cannot convert DateTimeOffset to NaiveDateTime"),
                None => None,
            };
            Ok(ColumnValue::DateTimeOffset(datetime))
        }
        ColumnData::Guid(value) => {
            let guid = match value {
                Some(val) => Some(
                    surrealdb::sql::Uuid::from_str(&val.to_string())
                        .expect("Cannot convert guid to uuid"),
                ),
                None => None,
            };
            Ok(ColumnValue::Uuid(guid))
        }
        ColumnData::Numeric(value) => {
            let numeric = match value {
                Some(_) => Decimal::from_sql(&column_data).expect("Cannot convert to Decimal"),
                None => None,
            };
            Ok(ColumnValue::Decimal(numeric))
        }
        _ => Err(anyhow::anyhow!("Column data not handled")),
    }
}
