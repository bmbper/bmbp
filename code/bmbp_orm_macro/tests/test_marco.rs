use serde::Deserialize;
use serde::Serialize;

use bmbp_app_common::BmbpTree;
use bmbp_app_common::BmbpValue;
use bmbp_app_common::{BmbpBaseModel, BmbpHashMap};
use bmbp_orm::BmbpScriptSql;
use bmbp_orm_macro::base;
use bmbp_orm_macro::orm;
use bmbp_orm_macro::tree;
use bmbp_orm_macro::{bmbp_value, method};

#[test]
fn test_orm() {
    #[orm]
    pub struct Demo {}
    println!("{:#?}", Demo::get_orm_fields())
}

#[test]
fn test_model() {
    #[base]
    #[method]
    #[derive(Default)]
    pub struct Demo {
        name: String,
    }
    let demo1 = Demo::default();
    println!("{:#?}", demo1.get_name())
}

#[test]
fn test_bmbp_value() {
    #[base]
    #[method]
    #[bmbp_value]
    #[derive(Default, Debug)]
    pub struct Demo {
        name: String,
    }
    let demo1 = Demo::default();
    let bv = BmbpValue::from(demo1);
    println!("{:#?}", bv);
}

#[test]
fn test_tree() {
    #[tree("organ")]
    #[base]
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
