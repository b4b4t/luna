use super::{column::Column, ColumnValue};
use crate::core::dao::table::TableDao;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    take: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<HashMap<String, Column>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<Vec<Vec<ColumnValue>>>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            condition: None,
            skip: None,
            take: None,
            columns: None,
            rows: None,
        }
    }

    pub fn from_dao(table: TableDao) -> Self {
        Self {
            name: table.name,
            condition: None,
            skip: None,
            take: None,
            columns: None,
            rows: None,
        }
    }

    pub fn get_table_name(&self) -> &str {
        return &self.name;
    }

    pub fn add_columns(&mut self, columns: Vec<Column>) {
        let mut cols: HashMap<String, Column> = HashMap::new();

        for col in columns {
            cols.insert(col.get_table_name().to_string(), col);
        }

        self.columns = Some(cols);
    }

    pub fn get_columns_iter(
        &self,
    ) -> Option<std::collections::hash_map::Values<'_, String, Column>> {
        if self.columns.is_none() {
            return None;
        }

        Some(self.columns.as_ref().unwrap().values())
    }

    pub fn get_column(&self, column_name: &str) -> Option<Column> {
        if self.columns.is_none() {
            return None;
        }

        let columns = self.columns.as_ref().unwrap();

        if columns.contains_key(column_name) {
            return Some(columns[column_name].clone());
        }

        None
    }

    pub fn get_rows(&self) -> Option<&Vec<Vec<ColumnValue>>> {
        self.rows.as_ref()
    }

    pub fn to_dao(&self) -> TableDao {
        let mut table = TableDao::new(self.name.clone());

        // Convert columns
        let mut columns = Vec::new();
        self.columns
            .as_ref()
            .expect("Table must have columns")
            .values()
            .into_iter()
            .for_each(|column| {
                columns.push(column.to_dao());
            });
        table.add_columns(columns);

        table
    }
}
