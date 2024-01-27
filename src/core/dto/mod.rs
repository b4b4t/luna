use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    name: String,
    tables: HashMap<String, Table>,
}

impl Model {
    pub fn new(model_name: &str) -> Self {
        Self {
            name: model_name.to_string(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<Vec<ColumnValue>>,
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
    foreign_key: Option<ForeignKey>,
}

impl Column {
    pub fn new(
        column_name: String,
        table_name: String,
        type_name: String,
        precision: u8,
        max_length: i16,
        foreign_key: Option<ForeignKey>,
    ) -> Self {
        Self {
            column_name,
            table_name,
            type_name,
            precision,
            max_length,
            foreign_key,
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

    pub fn is_foreign_key(&self) -> bool {
        return self.foreign_key.is_some();
    }

    pub fn get_foreign_key(&self) -> Option<&ForeignKey> {
        return self.foreign_key.as_ref();
    }
}

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
}
