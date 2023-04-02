use bmbp_orm_macro::orm;
#[orm]
pub struct Demo1 {
    name: String,
    id: String,
    #[skip]
    #[record]
    children: Vec<String>,
}

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

    #[test]
    fn it_works() {
        let mut demo = Demo1 {
            name: "XXX".to_string(),
            id: "XXX".to_string(),
            children: vec![],
        };
    }
}
