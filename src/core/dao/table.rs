use super::column::ColumnDao;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableDao {
    pub id: Option<Thing>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<HashMap<String, ColumnDao>>,
}

impl TableDao {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            columns: None,
        }
    }

    pub fn add_columns(&mut self, columns: Vec<ColumnDao>) {
        let mut cols: HashMap<String, ColumnDao> = HashMap::new();

        for col in columns {
            cols.insert(col.column_name.clone(), col);
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
}
