use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDao {
    pub id: Option<Thing>,
    pub name: String,
    pub model_name: String,
}

impl ModelDao {
    pub fn new(name: &str, model_name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            model_name: model_name.to_string(),
        }
    }
}
