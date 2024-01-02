use async_std::net::TcpStream;
use std::env;
use tiberius::{Client, Config};

use crate::core::dto::{Column, Table};

pub fn get_connection_string() -> String {
    env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set")
}

pub async fn get_client() -> anyhow::Result<Client<TcpStream>> {
    let connection_string = get_connection_string();
    let config = Config::from_ado_string(&connection_string)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = Client::connect(config, tcp).await?;

    Ok(client)
}

pub async fn get_tables() -> anyhow::Result<Vec<Table>> {
    let mut client = get_client().await?;
    let mut tables = Vec::<Table>::new();
    let query = "SELECT * FROM sys.tables where type_desc = 'USER_TABLE' order by name";

    let rows = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    for row in rows {
        let name: &str = row.get("name").unwrap();
        println!("Table : {}", name);

        tables.push(Table::new(name.to_string()));
    }

    Ok(tables)
}

pub async fn get_columns() -> anyhow::Result<Vec<Column>> {
    let mut client = get_client().await?;
    let mut columns = Vec::<Column>::new();
    let query = "select t.name as table_name, c.name as column_name, p.name as type_name, p.[precision], p.max_length from sys.columns c join sys.tables t on t.object_id = c.object_id join sys.types as p on c.system_type_id = p.system_type_id where t.type_desc = 'USER_TABLE' order by table_name, column_name";

    let rows = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    for row in rows {
        let column_name: &str = row.get("column_name").unwrap();
        let table_name: &str = row.get("table_name").unwrap();
        let type_name: &str = row.get("type_name").unwrap();
        let precision: u8 = row.get("precision").unwrap();
        let max_length: i16 = row.get("max_length").unwrap();
        println!(
            "Column : {}.{} {} ({},{})",
            table_name, column_name, type_name, max_length, precision
        );

        columns.push(Column::new(
            column_name.to_string(),
            table_name.to_string(),
            type_name.to_string(),
            precision,
            max_length,
        ));
    }

    Ok(columns)
}
