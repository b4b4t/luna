use luna_core::core::{
    dao::{column::ColumnDao, table::TableDao},
    dto::ForeignKey,
};

pub mod export_service_tests;
pub mod import_service_tests;

pub fn create_table(table_name: &str) -> TableDao {
    let columns = vec![ColumnDao::new(
        "col1".to_string(),
        table_name.to_string(),
        String::new(),
        0,
        0,
        None,
        1,
        true,
    )];
    let mut table = TableDao::new(table_name);
    table.add_columns(columns);

    table
}

pub fn create_fk_table(table_name: &str, fk_table_name: &str) -> TableDao {
    let columns = vec![
        create_column(table_name, "col1", 1, true),
        create_fk_column(table_name, fk_table_name, "col2", "col1", 2, true),
    ];
    let mut table = TableDao::new(table_name);
    table.add_columns(columns);

    table
}

pub fn create_two_fks_table(
    table_name: &str,
    fk_table_name1: &str,
    fk_table_name2: &str,
) -> TableDao {
    let columns = vec![
        create_column(table_name, "col1", 1, true),
        create_fk_column(table_name, fk_table_name1, "col2", "col1", 2, true),
        create_fk_column(table_name, fk_table_name2, "col3", "col1", 3, true),
    ];
    let mut table = TableDao::new(table_name);
    table.add_columns(columns);

    table
}

pub fn create_column(
    table_name: &str,
    column_name: &str,
    order: i16,
    is_primary_key: bool,
) -> ColumnDao {
    ColumnDao::new(
        column_name.to_string(),
        table_name.to_string(),
        String::new(),
        0,
        0,
        None,
        order,
        is_primary_key,
    )
}

pub fn create_fk_column(
    table_name: &str,
    fk_table_name: &str,
    column_name: &str,
    fk_column_name: &str,
    order: i16,
    is_primary_key: bool,
) -> ColumnDao {
    ColumnDao::new(
        column_name.to_string(),
        table_name.to_string(),
        String::new(),
        0,
        0,
        Some(create_fk(fk_table_name, fk_column_name)),
        order,
        is_primary_key,
    )
}

pub fn create_fk(table_name: &str, column_name: &str) -> ForeignKey {
    ForeignKey::new(
        column_name.to_string(),
        table_name.to_string(),
        String::new(),
    )
}
