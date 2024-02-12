use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::core::dto::ColumnValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableRowDao {
    pub id: Option<Thing>,
    pub row: Vec<ColumnValue>,
}

impl TableRowDao {
    pub fn new(row: Vec<ColumnValue>) -> Self {
        Self { id: None, row }
    }
}
