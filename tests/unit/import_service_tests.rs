use super::super::unit::{create_fk_table, create_table};
use luna_core::core::service::import_service::ImportService;

#[test]
fn test_sort_table() {
    let tables = vec![
        create_table("table1"),
        create_table("table2"),
        create_table("table3"),
        create_fk_table("table4", "table2"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[1].name, "table4");
}
