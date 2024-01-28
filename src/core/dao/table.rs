use super::column::ColumnDao;
use crate::core::dto::ColumnValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableDao {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<HashMap<String, ColumnDao>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Vec<ColumnValue>>>,
}

impl TableDao {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: None,
            rows: None,
        }
    }

    pub fn add_columns(&mut self, columns: Vec<ColumnDao>) {
        let mut cols: HashMap<String, ColumnDao> = HashMap::new();

        for col in columns {
            cols.insert(col.table_name.clone(), col);
        }

        self.columns = Some(cols);
    }

    pub fn get_columns_iter(
        &self,
    ) -> Option<std::collections::hash_map::Values<'_, String, ColumnDao>> {
        if self.columns.is_none() {
            return None;
        }

        Some(self.columns.as_ref().unwrap().values())
    }

    pub fn get_column(&self, column_name: &str) -> Option<ColumnDao> {
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
}
