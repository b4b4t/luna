use async_std::net::TcpStream;
use std::env;
use tiberius::{Client, Config};

use crate::core::dto::{Column, ForeignKey, Table};

pub mod query_builder;

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
        tables.push(Table::new(name.to_string()));
    }

    Ok(tables)
}

pub async fn get_columns() -> anyhow::Result<Vec<Column>> {
    let mut client = get_client().await?;
    let mut columns = Vec::<Column>::new();
    let query = "
        select 
            t.name as table_name, 
            c.name as column_name, 
            p.name as type_name, 
            p.[precision], 
            p.max_length,
            col2.column_id,
            col2.name as fk_col_name,
            tab2.name as fk_table_name,
            typ2.name as fk_type_name
        from sys.columns c 
        join sys.tables t on t.object_id = c.object_id 
        join sys.types as p on c.user_type_id = p.user_type_id 
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

    for row in rows {
        let column_name: &str = row.get("column_name").unwrap();
        let table_name: &str = row.get("table_name").unwrap();
        let type_name: &str = row.get("type_name").unwrap();
        let precision: u8 = row.get("precision").unwrap();
        let max_length: i16 = row.get("max_length").unwrap();
        let fk_col_name: Option<&str> = row.get("fk_col_name");
        let fk_table_name: Option<&str> = row.get("fk_table_name");
        let fk_type_name: Option<&str> = row.get("fk_type_name");
        // println!(
        //     "Column : {}.{} {} ({},{})",
        //     table_name, column_name, type_name, max_length, precision
        // );
        let foreign_key = match fk_col_name {
            Some(_) => Some(ForeignKey::new(
                fk_col_name.unwrap().to_string(),
                fk_table_name.unwrap().to_string(),
                fk_type_name.unwrap().to_string(),
            )),
            None => None,
        };
        columns.push(Column::new(
            column_name.to_string(),
            table_name.to_string(),
            type_name.to_string(),
            precision,
            max_length,
            foreign_key,
        ));
    }

    Ok(columns)
}

pub async fn execute_data_query(query: &str) -> anyhow::Result<Vec<String>> {
    let mut client = get_client().await?;
    let rows = client
        .query(query, &[&1i32])
        .await?
        .into_first_result()
        .await?;

    for row in rows {
        for col in row {}
    }
}
