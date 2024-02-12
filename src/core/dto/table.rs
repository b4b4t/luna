use super::{column::Column, ColumnValue};
use crate::core::dao::table::TableDao;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    id: Option<Thing>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    take: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<Column>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<Vec<Vec<ColumnValue>>>,
}

impl Table {
    pub fn from_dao(table: TableDao) -> Self {
        let columns = table
            .columns
            .expect("No column found in the table")
            .iter()
            .map(|(_, c)| Column::from_dao(c))
            .collect::<Vec<Column>>();

        Self {
            id: table.id,
            name: table.name,
            condition: None,
            skip: None,
            take: None,
            columns: Some(columns),
            rows: None,
        }
    }

    pub fn get_table_name(&self) -> &str {
        return &self.name;
    }

    pub fn add_columns(&mut self, columns: Vec<Column>) {
        self.columns = Some(columns);
    }

    pub fn get_columns(&self) -> Option<&Vec<Column>> {
        self.columns.as_ref()
    }

    pub fn get_column(&self, column_name: &str) -> Option<&Column> {
        if self.columns.is_none() {
            return None;
        }

        return self
            .columns
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.get_column_name() == column_name);
    }

    pub fn to_dao(&self) -> TableDao {
        let mut table = TableDao::new(&self.name);

        // Convert columns
        let mut columns = Vec::new();
        self.columns
            .as_ref()
            .expect("Table must have columns")
            .into_iter()
            .for_each(|column| {
                columns.push(column.to_dao());
            });
        table.add_columns(columns);

        table
    }
}
