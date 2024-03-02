use crate::{
    core::{
        dao::row::TableRowDao,
        data::{model_dal::ModelDal, table_dal::TableDal, table_row_dal::TableRowDal, SurrealDb},
        service::{
            build_model_from_database, check_model, fill_table_columns, read_model_from_file,
            sqlserver::{
                execute_data_query,
                query_builder::{DataQueryBuilder, Query},
            },
        },
    },
    println_error, println_success,
};
use std::time::Instant;

pub struct ExportService {}

impl ExportService {
    /// Export the data of the database into a surrealdb model
    pub async fn export_data(db: &SurrealDb, model_name: &str) -> anyhow::Result<()> {
        // Get db model
        let db_model = build_model_from_database(model_name).await?;

        // Get file model
        let mut file_model = read_model_from_file(model_name)?;

        // Check the model
        match check_model(&db_model, &file_model).await {
            Ok(_) => println_success!("Model is ok"),
            Err(err) => println_error!(
                "Error : database and file models are not matching => {}",
                err
            ),
        };

        // Fill columns in the file model
        fill_table_columns(&db_model, &mut file_model);

        // Save model
        println!("--> Saving model and tables");
        // let current: DateTime<Local> = Local::now();
        // let snapshot_name = current.format("%Y-%m-%d_%H:%M:%S").to_string();
        let model_dao = file_model.to_dao();
        let model_id = ModelDal::save_model(db, &model_dao).await?;

        // Get data from model
        println!("--> Fetching data");
        let now: Instant = Instant::now();
        for t in file_model.get_tables() {
            // Save tables
            let table_id = &TableDal::save_table(db, &t.to_dao()).await?;
            // Relate table to the table
            let _ = db
                .query("RELATE $model_id->owns->$table_id")
                .bind(("table_id", table_id))
                .bind(("model_id", &model_id))
                .await?;

            // Create query
            let mut columns = t.get_columns().expect("Columns must be fetched").clone();
            columns.sort_by_key(|c| c.get_order());

            let columns = columns
                .iter()
                .map(|c| c.get_column_name().to_string())
                .collect::<Vec<String>>();

            let query = DataQueryBuilder::new(t.get_table_name(), &columns).build();

            // Fetch data
            let rows = execute_data_query(&query).await?;

            println!(
                "|---> Saving {} data in model ({} rows to save)",
                t.get_table_name(),
                rows.len()
            );

            // Save data
            for row in rows {
                let row_id = TableRowDal::save_row(db, &TableRowDao::new(row)).await?;
                // Relate the row to the table
                let _ = db
                    .query("RELATE $table_id->contains->$row_id")
                    .bind(("table_id", table_id))
                    .bind(("row_id", row_id))
                    .await?;
            }
        }
        let elapsed = now.elapsed();
        println_success!("Data saved successfully in {:.2?}", elapsed);
        Ok(())
    }
}
