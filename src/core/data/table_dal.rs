use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use crate::core::dao::table::TableDao;

pub struct TableDal {}

impl TableDal {
    // Save a table
    pub async fn save_table(db: &Surreal<Db>, table_dao: &TableDao) -> anyhow::Result<Thing> {
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
