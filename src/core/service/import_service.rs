use surrealdb::sql::Thing;

use crate::{
    core::{
        dao::{column::ColumnDao, row::TableRowDao, table::TableDao},
        data::{table_dal::TableDal, table_row_dal::TableRowDal, SurrealDb},
        service::sqlserver_provider::query_builder::Query,
    },
    println_error, println_success,
};

use super::{export_service::Provider, sqlserver_provider::query_builder::DataInsertQueryBuilder};

pub struct ImportService {}

impl ImportService {
    pub async fn import_data(
        db: &SurrealDb,
        model_id: &Thing,
        provider: &mut Box<dyn Provider>,
    ) -> anyhow::Result<()> {
        // Get the tables
        let tables = TableDal::get_tables_by_model_id(db, model_id).await?;

        // Order tables
        let tables = ImportService::sort_tables(&tables);

        // Open provider connection
        provider.open_connection()?;

        // Get the data by table
        for table in &tables.unwrap() {
            let rows: Vec<TableRowDao> =
                TableRowDal::get_rows_by_table(db, table.id.as_ref().unwrap()).await?;

            let mut columns = table
                .columns
                .as_ref()
                .unwrap()
                .iter()
                .map(|(_, c)| c)
                .collect::<Vec<&ColumnDao>>();
            columns.sort_by_key(|c| c.order);

            let columns = columns
                .iter()
                .map(|c| c.column_name.to_string())
                .collect::<Vec<String>>();

            let insert_queries = rows
                .into_iter()
                .map(|r| DataInsertQueryBuilder::new(&table.name, &columns, r.row));

            provider.send(&format!("-- Table {}", table.name))?;
            for insert in insert_queries {
                provider.send(&insert.build())?;
            }
        }

        println_success!("Data imported");

        Ok(())
    }

    /// Sort a list of tables
    pub fn sort_tables(tables: &Vec<TableDao>) -> anyhow::Result<Vec<TableDao>> {
        let mut is_success = true;
        let mut ordered_tables: Vec<TableDao> = Vec::new();

        for table in tables.iter() {
            // Get the position
            let mut position = match position_table_by_name(&ordered_tables, &table.name) {
                Some(t_position) => t_position,
                None => ordered_tables.len(),
            };

            // Add the foreign tables before
            for (_, column) in table.columns.as_ref().unwrap() {
                if column.foreign_key.is_none() {
                    continue;
                }
                let foreign_key = column.foreign_key.as_ref().unwrap();

                // Find table to know if was already added
                if !any_table_by_name(&ordered_tables, &foreign_key.table_name) {
                    match find_table_by_name(&tables, &foreign_key.table_name) {
                        // Add the foreign table
                        Some(ft) => {
                            ordered_tables.insert(position, ft.clone());
                            position += 1;
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
                    // If the table is found in the ordered vec, get the position
                    position =
                        match position_table_by_name(&ordered_tables, &foreign_key.table_name) {
                            Some(t_position) => t_position,
                            None => ordered_tables.len(),
                        };
                }
            }

            if !any_table_by_name(&ordered_tables, &table.name) {
                ordered_tables.insert(position, table.clone());
            }
        }

        if !is_success {
            return Err(anyhow::anyhow!("Error when ordering tables"));
        }

        Ok(ordered_tables)
    }
}

fn find_table_by_name<'a>(tables: &'a Vec<TableDao>, table_name: &str) -> Option<&'a TableDao> {
    tables.iter().find(|t| t.name == table_name)
}

fn position_table_by_name<'a>(tables: &'a Vec<TableDao>, table_name: &str) -> Option<usize> {
    tables.iter().position(|t| t.name == table_name)
}

fn any_table_by_name<'a>(tables: &'a Vec<TableDao>, table_name: &str) -> bool {
    tables.iter().any(|t| t.name == table_name)
}
