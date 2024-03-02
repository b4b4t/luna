use super::SurrealDb;
use crate::core::dao::table::TableDao;
use surrealdb::sql::Thing;

pub struct TableDal {}

impl TableDal {
    pub async fn get_tables_by_model_id(
        db: &SurrealDb,
        model_id: &Thing,
    ) -> anyhow::Result<Vec<TableDao>> {
        let tables: Vec<TableDao> = db
            .query("select * from $model_id->owns->table")
            .bind(("model_id", model_id))
            .await?
            .take(0)?;

        Ok(tables)
    }

    // Save a table
    pub async fn save_table(db: &SurrealDb, table_dao: &TableDao) -> anyhow::Result<Thing> {
        let t_record: Vec<TableDao> = db.create("table").content(table_dao).await?;
        let table_id = t_record
            .first()
            .expect("Cannot create table")
            .id
            .clone()
            .expect("Cannot get table id");

        Ok(table_id)
    }
}
