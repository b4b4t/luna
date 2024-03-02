use super::SurrealDb;
use crate::core::dao::model::ModelDao;
use surrealdb::sql::Thing;

pub struct ModelDal {}

impl ModelDal {
    /// Save a model
    pub async fn save_model(db: &SurrealDb, model_dao: &ModelDao) -> anyhow::Result<Thing> {
        let results: Vec<ModelDao> = db.create("model").content(model_dao).await?;
        let model = results.first().expect("Model not created");
        let model_id = model.id.clone().expect("Cannot get model id");

        Ok(model_id)
    }

    pub async fn get_models(db: &SurrealDb) -> anyhow::Result<Vec<ModelDao>> {
        let models: Vec<ModelDao> = db.select("model").await?;

        Ok(models)
    }

    // pub async fn get_model(db: &SurrealDb, model_id: Thing) -> anyhow::Result<Option<ModelDao>> {
    //     let model: Option<ModelDao> = db.select(("model", model_id)).await?;

    //     Ok(model)
    // }
}
