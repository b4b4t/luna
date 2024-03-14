use crate::unit::create_two_fks_table;

use super::super::unit::{create_fk_table, create_table};
use luna_core::core::service::import_service::ImportService;

#[test]
fn sort_tables_without_fk() {
    let tables = vec![
        create_table("table1"),
        create_table("table2"),
        create_table("table3"),
        create_table("table4"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].name, "table1");
    assert_eq!(ordered_tables[1].name, "table2");
    assert_eq!(ordered_tables[2].name, "table3");
    assert_eq!(ordered_tables[3].name, "table4");
}

#[test]
fn sort_tables_one_fk() {
    let tables = vec![
        create_table("table1"),
        create_table("table2"),
        create_table("table3"),
        create_fk_table("table4", "table2"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].name, "table1");
    assert_eq!(ordered_tables[1].name, "table2");
    assert_eq!(ordered_tables[2].name, "table4");
    assert_eq!(ordered_tables[3].name, "table3");
}

#[test]
fn sort_tables_two_fks() {
    let tables = vec![
        create_fk_table("table1", "table3"),
        create_table("table2"),
        create_table("table3"),
        create_fk_table("table4", "table2"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].name, "table3");
    assert_eq!(ordered_tables[1].name, "table1");
    assert_eq!(ordered_tables[2].name, "table2");
    assert_eq!(ordered_tables[3].name, "table4");
}

#[test]
fn sort_tables_with_two_fks_same_table() {
    let tables = vec![
        create_table("table1"),
        create_two_fks_table("table2", "table1", "table4"),
        create_table("table3"),
        create_table("table4"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].name, "table1");
    assert_eq!(ordered_tables[1].name, "table4");
    assert_eq!(ordered_tables[2].name, "table2");
    assert_eq!(ordered_tables[3].name, "table3");
}

#[test]
fn sort_tables_with_two_fks_same_table_and_one_fk() {
    let tables = vec![
        create_table("table1"),
        create_two_fks_table("table2", "table1", "table4"),
        create_table("table3"),
        create_table("table4"),
        create_fk_table("table5", "table1"),
    ];

    let ordered_tables = ImportService::sort_tables(&tables).unwrap();

    assert_eq!(ordered_tables[0].name, "table1");
    assert_eq!(ordered_tables[1].name, "table5");
    assert_eq!(ordered_tables[2].name, "table2");
    assert_eq!(ordered_tables[3].name, "table4");
    assert_eq!(ordered_tables[4].name, "table3");
}
