use super::table::Table;
use crate::core::dao::model::ModelDao;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    name: String,
    tables: Vec<Table>,
}

impl Model {
    pub fn new(model_name: &str) -> Self {
        Self {
            name: model_name.to_string(),
            tables: Vec::new(),
        }
    }

    pub fn add(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_tables(&self) -> &Vec<Table> {
        &self.tables
    }

    pub fn get_tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    pub fn get_model_name(&self) -> &str {
        &self.name
    }

    pub fn to_dao(&self) -> ModelDao {
        let mut model = ModelDao::new(&self.name);
        for table in &self.tables {
            model.add(table.to_dao());
        }

        model
    }
}
