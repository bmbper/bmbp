use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use bmbp_rdbc_orm::{BmbpOrmRdbcTree, RdbcModel};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrgan {
    organ_type: Option<BmbpRdbcOrganType>,
}

impl RdbcModel for BmbpRbacOrgan {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN".to_string()
    }
    fn get_table_fields() -> Vec<String> {
        vec![
            "organ_type".to_string(),
        ]
    }
}

pub type BmbpRbacOrganTreeNode = BmbpOrmRdbcTree<BmbpRbacOrgan>;


#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpRdbcOrganType {
    REGION = 0,
    GROUP = 1,
    UNIT = 2,
    DEPT = 3,
    POST = 4,
    PERSON = 5,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganRegion {
    organ_id: Option<String>,
}
impl RdbcModel for BmbpRbacOrganRegion{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganGroup {
    organ_id: Option<String>,
}

impl RdbcModel for BmbpRbacOrganGroup{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganUnit {
    organ_id: Option<String>,
}

impl RdbcModel for BmbpRbacOrganUnit{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganDept {
    organ_id: Option<String>,
}
impl RdbcModel for BmbpRbacOrganDept{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganPost {
    organ_id: Option<String>,
}
impl RdbcModel for BmbpRbacOrganPost{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRbacOrganPerson {
    organ_id: Option<String>,
}
impl RdbcModel for BmbpRbacOrganPerson{
    fn get_table_name() -> String {
        "".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![]
    }
}