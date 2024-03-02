use crate::core::dao::column::ColumnDao;

use super::ForeignKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    column_name: String,
    table_name: Option<String>,
    type_name: Option<String>,
    precision: Option<u8>,
    max_length: Option<i16>,
    foreign_key: Option<ForeignKey>,
    order: i16,
}

impl Column {
    // pub fn new(
    //     column_name: String,
    //     table_name: String,
    //     type_name: String,
    //     precision: u8,
    //     max_length: i16,
    //     foreign_key: Option<ForeignKey>,
    //     order: i16,
    // ) -> Self {
    //     Self {
    //         column_name,
    //         table_name: Some(table_name),
    //         type_name: Some(type_name),
    //         precision: Some(precision),
    //         max_length: Some(max_length),
    //         foreign_key,
    //         order,
    //     }
    // }

    pub fn from_dao(column: &ColumnDao) -> Self {
        Self {
            column_name: column.column_name.clone(),
            table_name: Some(column.table_name.clone()),
            type_name: Some(column.type_name.clone()),
            precision: Some(column.precision),
            max_length: Some(column.max_length),
            foreign_key: column.foreign_key.clone(),
            order: column.order.clone(),
        }
    }

    pub fn get_column_name(&self) -> &str {
        return &self.column_name;
    }

    // pub fn get_table_name(&self) -> &str {
    //     return &self.column_name;
    // }

    pub fn get_type_name(&self) -> &str {
        return &self.column_name;
    }

    pub fn get_order(&self) -> i16 {
        return self.order;
    }

    // pub fn is_foreign_key(&self) -> bool {
    //     return self.foreign_key.is_some();
    // }

    // pub fn get_foreign_key(&self) -> Option<&ForeignKey> {
    //     return self.foreign_key.as_ref();
    // }

    pub fn to_dao(&self) -> ColumnDao {
        ColumnDao::new(
            self.column_name.clone(),
            self.table_name
                .as_ref()
                .expect("Missing table name")
                .to_string(),
            self.type_name
                .as_ref()
                .expect("Missing type name")
                .to_string(),
            self.precision.as_ref().expect("Missing precision").clone(),
            self.max_length
                .as_ref()
                .expect("MIssing max length")
                .clone(),
            self.foreign_key.clone(),
            self.order.clone(),
        )
    }
}
