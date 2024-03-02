use crate::core::dto::ForeignKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnDao {
    pub column_name: String,
    pub table_name: String,
    pub type_name: String,
    pub precision: u8,
    pub max_length: i16,
    pub foreign_key: Option<ForeignKey>,
    pub order: i16,
}

impl ColumnDao {
    pub fn new(
        column_name: String,
        table_name: String,
        type_name: String,
        precision: u8,
        max_length: i16,
        foreign_key: Option<ForeignKey>,
        order: i16,
    ) -> Self {
        Self {
            column_name,
            table_name,
            type_name,
            precision,
            max_length,
            foreign_key,
            order,
        }
    }
}
