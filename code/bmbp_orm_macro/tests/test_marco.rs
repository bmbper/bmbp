use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use bmbp_orm::BmbpScriptSql;
use bmbp_orm_macro::method;
use bmbp_orm_macro::model;
use bmbp_orm_macro::orm;
use bmbp_orm_macro::page;
use bmbp_orm_macro::tree;
use bmbp_types::BmbpTree;
use bmbp_types::BmbpValue;

#[test]
fn test_orm() {
    #[orm]
    pub struct Demo {}
    println!("{:#?}", Demo::get_orm_fields())
}

#[test]
fn test_model() {
    #[model]
    #[method]
    #[derive(Default)]
    pub struct Demo {
        name: String,
    }
    let demo1 = Demo::default();
    println!("{:#?}", demo1.get_name())
}

#[test]
fn test_page() {
    #[page]
    #[method]
    #[derive(Default)]
    pub struct Demo {
        name: String,
    }
    let mut demo1 = Demo::default();
    demo1.set_page_no(12usize);
    println!("{:#?}", demo1.get_page_no())
}

#[test]
fn test_validator() {}

#[test]
fn test_tree() {
    #[tree("organ")]
    #[model]
    #[method]
    #[orm(table = RBAC_ORGAN, id = r_id, exclude = organ_children | organ_title)]
    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct Demo {
        demo: String,
    }
    let mut demo1 = Demo::default();
    demo1.set_organ_id("1234556677".to_string());
    demo1.set_demo("1234556677".to_string());
    println!("{:#?}", demo1);
    println!("{:#?}", Demo::get_orm_fields());
    println!("{:#?}", Demo::get_orm_table())
}
