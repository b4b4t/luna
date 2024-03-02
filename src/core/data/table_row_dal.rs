use super::SurrealDb;
use crate::core::dao::row::TableRowDao;
use surrealdb::sql::Thing;

pub struct TableRowDal {}

impl TableRowDal {
    // Get a list of row by table
    pub async fn get_rows_by_table(
        db: &SurrealDb,
        table_id: &Thing,
    ) -> anyhow::Result<Vec<TableRowDao>> {
        let rows: Vec<TableRowDao> = db
            .query("select * from $table_id->contains->row")
            .bind(("table_id", table_id))
            .await?
            .take(0)?;

        Ok(rows)
    }

    // Save a row
    pub async fn save_row(db: &SurrealDb, row_dao: &TableRowDao) -> anyhow::Result<Thing> {
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
