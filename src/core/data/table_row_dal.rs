use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use crate::core::dao::row::TableRowDao;

pub struct TableRowDal {}

impl TableRowDal {
    // Save a row
    pub async fn save_row(db: &Surreal<Db>, row_dao: &TableRowDao) -> anyhow::Result<Thing> {
        let r_record: Vec<TableRowDao> = db.create("row").content(row_dao).await?;
        let row_id = r_record
            .first()
            .expect("Cannot create row")
            .id
            .clone()
            .expect("Cannot get row id");

        Ok(row_id)
    }
}
