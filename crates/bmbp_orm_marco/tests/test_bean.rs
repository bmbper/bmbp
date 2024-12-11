use bmbp_orm_marco::{table, tree_table};
use bmbp_sql::RdbcColumnIdent;
use bmbp_sql::RdbcTableIdent;
use bmbp_util::BmbpTree;
use serde::Deserialize;
use serde::Serialize;
#[test]
fn test_bean() {
    #[table]
    pub struct Demo {}
    let demo = Demo::default();
    println!("{}", demo.data_id);
}

#[test]
fn test_tree_bean() {
    #[tree_table(table=DAMP_DD,tree=app)]
    pub struct TreeDemo {}
    let mut demo = TreeDemo::default();
    demo.app_code = "DDDD".to_string();
    println!("===>{}", demo.app_children.len());
    println!("===>{}", TreeDemo::primary_key().name());
    let names = TreeDemo::columns()
        .iter()
        .map(|x| x.name())
        .collect::<Vec<_>>();
    println!("===>{:?}", names);
}
