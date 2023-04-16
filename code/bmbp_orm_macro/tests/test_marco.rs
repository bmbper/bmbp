use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use bmbp_orm::BmbpScriptSql;
use bmbp_orm_macro::model;
use bmbp_orm_macro::orm;
use bmbp_orm_macro::page;
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
    pub struct Demo {
        name: String,
    }
    let demo1 = Demo::default();
    println!("{:#?}", demo1.get_name())
}

#[test]
fn test_page() {
    #[page]
    pub struct Demo {
        name: String,
    }
    let mut demo1 = Demo::default();
    demo1.set_page_no(12usize);
    println!("{:#?}", demo1.get_page_no())
}

#[test]
fn test_validator() {}
