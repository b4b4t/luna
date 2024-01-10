use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    tables: HashMap<String, Table>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add(&mut self, table: Table) {
        let _ = &self
            .tables
            .insert(table.get_table_name().to_string(), table);
    }

    pub fn get(&self, table_name: &str) -> Option<Table> {
        let tables = &self.tables;

        if tables.contains_key(table_name) {
            return Some(tables[table_name].clone());
        }

        None
    }

    pub fn get_tables_iter(&self) -> std::collections::hash_map::Values<'_, String, Table> {
        self.tables.values()
    }
}

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
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            condition: None,
            skip: None,
            take: None,
            columns: None,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    column_name: String,
    table_name: String,
    type_name: String,
    precision: u8,
    max_length: i16,
}

impl Column {
    pub fn new(
        column_name: String,
        table_name: String,
        type_name: String,
        precision: u8,
        max_length: i16,
    ) -> Self {
        Self {
            column_name,
            table_name,
            type_name,
            precision,
            max_length,
        }
    }

    pub fn get_column_name(&self) -> &str {
        return &self.column_name;
    }

    pub fn get_table_name(&self) -> &str {
        return &self.column_name;
    }

    pub fn get_type_name(&self) -> &str {
        return &self.column_name;
    }
}
