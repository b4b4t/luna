use crate::{
    core::{
        dao::row::TableRowDao,
        data::{model_dal::ModelDal, table_dal::TableDal, table_row_dal::TableRowDal, SurrealDb},
        dto::table::Table,
        service::{
            build_model_from_database, check_model, fill_table_columns, read_model_from_file,
            sqlserver_provider::{
                execute_data_query,
                query_builder::{DataQueryBuilder, Query},
            },
        },
    },
    println_error, println_success,
};
use std::time::Instant;

pub trait Provider {
    fn open_connection(&mut self) -> anyhow::Result<()>;
    fn send(&mut self, data: &str) -> anyhow::Result<()>;
}

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
        let tables = ExportService::sort_tables(file_model.get_tables())?;

        println!("--> Fetching data");
        let now: Instant = Instant::now();
        for t in tables {
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

            let primary_key = columns
                .iter()
                .filter(|c| c.is_primary_key())
                .map(|c| c.get_column_name().to_string())
                .collect::<Vec<String>>();

            let columns = columns
                .iter()
                .map(|c| c.get_column_name().to_string())
                .collect::<Vec<String>>();

            let query = DataQueryBuilder::new(
                t.get_table_name(),
                &columns,
                primary_key,
                t.get_skip(),
                t.get_take(),
                t.get_predicate(),
            )
            .build();

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

    /// Sort a list of tables
    pub fn sort_tables(tables: &Vec<Table>) -> anyhow::Result<Vec<Table>> {
        let mut is_success = true;
        let mut ordered_tables: Vec<Table> = Vec::new();

        for table in tables.iter() {
            println!("-> Table {}", table.get_table_name());

            // Get the position
            let mut position =
                match position_table_by_name(&ordered_tables, &table.get_table_name()) {
                    Some(t_position) => t_position,
                    None => ordered_tables.len(),
                };

            // Add the foreign tables after
            for column in table.get_columns().unwrap() {
                if !column.is_foreign_key() {
                    continue;
                }
                let foreign_key = column.get_foreign_key().unwrap();
                println!("--> Foreign key {}", column.get_column_name());
                // Find table to know if was already added
                if !any_table_by_name(&ordered_tables, &foreign_key.table_name) {
                    match find_table_by_name(&tables, &foreign_key.table_name) {
                        // Add the foreign table
                        Some(ft) => {
                            ordered_tables.insert(position, ft.clone());
                        }
                        None => {
                            println_error!(
                                "Cannot find the table {} in the model",
                                foreign_key.table_name
                            );
                            is_success = false;
                        }
                    }
                } else {
                    // If the table is found in the ordered vec, get the position before
                    position =
                        match position_table_by_name(&ordered_tables, &foreign_key.table_name) {
                            Some(t_position) => t_position,
                            None => 0,
                        };
                }
            }

            if !any_table_by_name(&ordered_tables, &table.get_table_name()) {
                ordered_tables.insert(position, table.clone());
            }
        }

        if !is_success {
            return Err(anyhow::anyhow!("Error when ordering tables"));
        }

        Ok(ordered_tables)
    }
}

/// Find a table by name
fn find_table_by_name<'a>(tables: &'a Vec<Table>, table_name: &str) -> Option<&'a Table> {
    tables.iter().find(|t| t.get_table_name() == table_name)
}

/// Find a table position by name
fn position_table_by_name<'a>(tables: &'a Vec<Table>, table_name: &str) -> Option<usize> {
    tables.iter().position(|t| t.get_table_name() == table_name)
}

/// Check if it exists a table with the specified name
fn any_table_by_name<'a>(tables: &'a Vec<Table>, table_name: &str) -> bool {
    tables.iter().any(|t| t.get_table_name() == table_name)
}
