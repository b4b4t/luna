use crate::{
    core::{
        dao::{column::ColumnDao, table::TableDao},
        dto::{ColumnValue, ForeignKey},
    },
    println_error,
};

use std::env;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use self::mappers::to_column_value;

pub mod mappers;
pub mod provider;
pub mod query_builder;

pub fn get_connection_string() -> String {
    env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set")
}

/// Get a SqlServer client
pub async fn get_client() -> anyhow::Result<Client<Compat<TcpStream>>> {
    let connection_string = get_connection_string();
    let config = Config::from_ado_string(&connection_string)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}

/// Get all tables of the targeted database
pub async fn get_tables() -> anyhow::Result<Vec<TableDao>> {
    let mut client = get_client().await?;
    let mut tables = Vec::<TableDao>::new();
    let query = "SELECT * FROM sys.tables where type_desc = 'USER_TABLE' order by name";

    let rows = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    for row in rows {
        let name: &str = row.get("name").unwrap();
        tables.push(TableDao::new(name));
    }

    Ok(tables)
}

/// Get all the columns of the targated table
pub async fn get_columns() -> anyhow::Result<Vec<ColumnDao>> {
    let mut client = get_client().await?;
    let mut columns = Vec::<ColumnDao>::new();
    let query: &str = "
        select 
            t.name as table_name, 
            c.name as column_name, 
            p.name as type_name, 
            p.[precision], 
            p.max_length,
            col2.column_id,
            col2.name as fk_col_name,
            tab2.name as fk_table_name,
            typ2.name as fk_type_name,
            case when indcol.key_ordinal is not null then 1 else 0 end as is_primary_key
        from sys.columns c 
        join sys.tables t on t.object_id = c.object_id 
        join sys.types as p on c.user_type_id = p.user_type_id 
        left join sys.indexes ind on ind.object_id = t.object_id and ind.is_primary_key = 1
        left join sys.index_columns indcol on indcol.object_id = t.object_id and indcol.column_id = c.column_id and ind.index_id = indcol.index_id
        left join sys.foreign_key_columns fkc on fkc.parent_column_id = c.column_id and fkc.parent_object_id = c.object_id
        left join sys.tables tab2
            on tab2.object_id = fkc.referenced_object_id
        left join sys.schemas sch2
            on tab2.schema_id = sch2.schema_id
        left join sys.columns col2
            on col2.column_id = fkc.referenced_column_id AND col2.object_id = tab2.object_id
        left join sys.types as typ2 
            on col2.user_type_id = typ2.user_type_id 
        where t.type_desc = 'USER_TABLE' 
        order by table_name, column_name";

    let rows = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    let mut order = 0;
    for row in rows {
        let column_name: &str = row.get("column_name").unwrap();
        let table_name: &str = row.get("table_name").unwrap();
        let type_name: &str = row.get("type_name").unwrap();
        let precision: u8 = row.get("precision").unwrap();
        let max_length: i16 = row.get("max_length").unwrap();
        let fk_col_name: Option<&str> = row.get("fk_col_name");
        let fk_table_name: Option<&str> = row.get("fk_table_name");
        let fk_type_name: Option<&str> = row.get("fk_type_name");
        let is_primary_key: i32 = row.get("is_primary_key").unwrap();
        let foreign_key = match fk_col_name {
            Some(_) => Some(ForeignKey::new(
                fk_col_name.unwrap().to_string(),
                fk_table_name.unwrap().to_string(),
                fk_type_name.unwrap().to_string(),
            )),
            None => None,
        };

        columns.push(ColumnDao::new(
            column_name.to_string(),
            table_name.to_string(),
            type_name.to_string(),
            precision,
            max_length,
            foreign_key,
            order,
            is_primary_key == 1,
        ));

        order += 1;
    }

    Ok(columns)
}

/// Execute a query on the SqlServer database
pub async fn execute_data_query(query: &str) -> anyhow::Result<Vec<Vec<ColumnValue>>> {
    let mut client = get_client().await?;
    // println!("query : {}", query);
    // println!("Execute query : {}", query);
    let rows: Vec<tiberius::Row> = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    let mut data: Vec<Vec<ColumnValue>> = Vec::new();
    let mut line = 0;
    // Read rows
    for row in rows {
        line += 1;
        let mut column = 0;
        let mut row_data: Vec<ColumnValue> = Vec::new();
        // println!("line : {}", line);
        // Read columns
        for col in row {
            column += 1;
            // println!("column : {}", column);
            // Convert to column value
            match to_column_value(col) {
                Ok(value) => {
                    row_data.push(value);
                }
                Err(error) => println_error!("[{}][{}] : {}", line, column, error),
            }
        }
        data.push(row_data);
    }

    Ok(data)
}
