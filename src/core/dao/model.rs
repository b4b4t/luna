use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::table::TableDao;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDao {
    name: String,
    tables: HashMap<String, TableDao>,
}

impl ModelDao {
    pub fn new(model_name: &str) -> Self {
        Self {
            name: model_name.to_string(),
            tables: HashMap::new(),
        }
    }

    pub fn add(&mut self, table: TableDao) {
        let _ = &self.tables.insert(table.name.clone(), table);
    }

    pub fn get(&self, table_name: &str) -> Option<TableDao> {
        let tables = &self.tables;

        if tables.contains_key(table_name) {
            return Some(tables[table_name].clone());
        }

        None
    }

    pub fn get_tables_iter(&self) -> std::collections::hash_map::Values<'_, String, TableDao> {
        self.tables.values()
    }

    pub fn get_model_name(&self) -> &str {
        &self.name
    }
}
