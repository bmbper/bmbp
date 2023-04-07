use serde::{Deserialize, Serialize};

use bmbp_orm::BmbpScriptSql;
use bmbp_orm_macro::{model, orm};

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
pub struct Demo2 {}

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

    use crate::{Demo1, Demo8};

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
        let mut demo2 = Demo8::default();
        demo2.set_name("xxxx".to_string());
        println!("=========>:{}", demo2.get_name())
    }
}
