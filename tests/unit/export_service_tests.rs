use luna_core::core::{dto::table::Table, service::export_service::ExportService};

use super::super::unit::{create_fk_table, create_table};

#[test]
fn test_sort_table() {
    let tables = vec![
        Table::from_dao(create_table("table1")),
        Table::from_dao(create_table("table2")),
        Table::from_dao(create_table("table3")),
        Table::from_dao(create_fk_table("table4", "table2")),
    ];

    let ordered_tables = ExportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].get_table_name(), "table1");
    assert_eq!(ordered_tables[1].get_table_name(), "table4");
    assert_eq!(ordered_tables[2].get_table_name(), "table2");
    assert_eq!(ordered_tables[3].get_table_name(), "table3");
}
