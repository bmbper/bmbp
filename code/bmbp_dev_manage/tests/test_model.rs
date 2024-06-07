use bmbp_dev_manage::BmbpDevTable;

#[test]
fn test_model() {
    let _ = BmbpDevTable::default();
    assert_eq!(BmbpDevTable::get_rdbc_table_name(), "BMBP_DEV_TABLE")
}
