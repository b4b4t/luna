use crate::core::data::SurrealDb;

pub struct ImportService {}

impl ImportService {
    pub async fn import_data(db: &SurrealDb, model_name: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
