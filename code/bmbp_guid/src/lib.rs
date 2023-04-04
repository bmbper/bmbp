use bmbp_orm::BmbpScriptSql;
use serde::{Deserialize, Serialize};

use bmbp_orm_macro::orm;

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

#[orm("rbac_organ_name", "recordId")]
pub struct Demo8 {}

#[cfg(test)]
mod tests {
    use crate::Demo1;
    use bmbp_orm::BmbpScriptSql;
    #[test]
    fn it_works() {
        let mut demo = Demo1::default();
        let mut scr = BmbpScriptSql::new();
        let a: Vec<String> = vec![];
        let b: Vec<String> = vec![];
        for (index, item) in a.as_slice().into_iter().enumerate() {
            let m = b.get(index);
            match m {
                Some(v) => {}
                None => {}
            }
        }

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
}
