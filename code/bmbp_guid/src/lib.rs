use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use bmbp_orm::BmbpScriptSql;
use bmbp_orm_macro::{model, orm};
use bmbp_types::BmbpValue;

#[orm(id = "recordId")]
pub struct Demo1 {
    name: String,
    id: String,
    #[skip]
    #[record]
    children: Vec<String>,
}

impl Demo1 {}

#[orm(table = "rbac_organ_name")]
pub struct Demo2 {
    gooo: Option<String>,
}
#[orm(table = "rbac_organ_name", id = "record_id")]
pub struct Demo3 {}

#[orm(rbac_organ_name)]
pub struct Demo4 {}

#[orm("rbac_organ_name")]
pub struct Demo5 {}

#[orm(rbac_organ_name, recordId)]
pub struct Demo6 {}

#[orm("rbac_organ_name", recordId)]
pub struct Demo7 {}

#[model]
pub struct Demo8 {
    name: String,
}

#[cfg(test)]
mod tests {
    use bmbp_orm::BmbpScriptSql;
    use bmbp_types::BmbpValue;

    use crate::{Demo1, Demo2, Demo8};

    #[test]
    fn test_orm() {
        let mut demo = Demo1::default();
        let mut scr = BmbpScriptSql::new();
        println!("{}", Demo1::get_orm_table());
        println!("{:#?}", Demo1::get_orm_fields());
        println!("===>{:#?}", Demo1::query_sql().to_sql_string());
        println!("===>{:#?}", Demo1::query_camel_sql().to_sql_string());
        println!("===>{:#?}", Demo1::update_sql().to_sql_string());
        println!("===>{:#?}", Demo1::update_camel_sql().to_sql_string());
        println!("===>{:#?}", Demo1::insert_sql().to_sql_string());
        println!("===>{:#?}", Demo1::insert_camel_sql().to_sql_string());
        println!("===>{:#?}", Demo1::delete_sql().to_sql_string());
        println!("===>{:#?}", Demo1::delete_one_sql().to_sql_string());
        println!("===>{:#?}", Demo1::delete_by_id_sql().to_sql_string());
    }

    #[test]
    fn test_model() {
        let mut demo2 = Demo2::default();
        demo2.set_r_id("xxxx".to_string());
        let bmbp = <Demo2 as Into<BmbpValue>>::into(demo2);
        let v = serde_json::to_string(&bmbp).unwrap();
        println!("======>:{:#?}", v);
    }
}
