use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    tables: Vec<Table>,
}

impl Model {
    pub fn new() -> Self {
        Self { tables: vec![] }
    }

    pub fn add(&mut self, table: Table) {
        let _ = &self.tables.push(table);
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
    #[serde(skip_serializing)]
    columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            condition: None,
            skip: None,
            take: None,
            columns: vec![],
        }
    }

    pub fn get_table_name(&self) -> &str {
        return &self.name;
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
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
