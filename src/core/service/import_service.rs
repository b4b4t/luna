use surrealdb::{engine::local::Db, Surreal};

pub struct ImportService {}

impl ImportService {
    pub async fn import_data(db: &Surreal<Db>, model_name: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
