use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use crate::core::dao::model::ModelDao;

pub struct ModelDal {}

impl ModelDal {
    /// Save a model
    pub async fn save_model(db: &Surreal<Db>, model_dao: &ModelDao) -> anyhow::Result<Thing> {
        let results: Vec<ModelDao> = db.create("model").content(model_dao).await?;
        let model = results.first().expect("Model not created");
        let model_id = model.id.clone().expect("Cannot get model id");

        Ok(model_id)
    }
}
