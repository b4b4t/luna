use std::fmt::Display;

use super::table::Table;
use crate::core::dao::model::ModelDao;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    id: Option<Thing>,
    name: Option<String>,
    model_name: String,
    tables: Vec<Table>,
}

impl Model {
    pub fn new(name: &str, model_name: &str) -> Self {
        Self {
            id: None,
            name: Some(name.to_string()),
            model_name: model_name.to_string(),
            tables: Vec::new(),
        }
    }

    pub fn add(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }

    pub fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables
            .iter()
            .find(|t| t.get_table_name() == table_name)
    }

    pub fn get_tables(&self) -> &Vec<Table> {
        &self.tables
    }

    pub fn get_tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    pub fn to_dao(&self) -> ModelDao {
        ModelDao::new(
            &self.name.as_ref().unwrap_or(&self.model_name),
            &self.model_name,
        )
    }

    pub fn from_dao(model: &ModelDao) -> Self {
        Self {
            id: model.id.clone(),
            model_name: model.model_name.clone(),
            name: Some(model.name.clone()),
            tables: Vec::new(),
        }
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self.name {
            Some(n) => n,
            None => &self.model_name,
        };
        write!(f, "{}", name)
    }
}
