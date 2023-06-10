use std::string::ToString;

use serde::{Deserialize, Serialize};

use bmbp_orm_ins::BmbpScriptSql;
use bmbp_orm_macro::{base, bmbp_value, method, orm, tree};
use bmbp_types::BmbpBaseModel;
use bmbp_types::{BmbpHashMap, BmbpTree, BmbpValue};

#[method]
#[bmbp_value]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrganQueryParam {
    r_id: String,
    organ_parent_id: String,
    organ_id: String,
}

#[tree(organ)]
#[base]
#[method]
#[bmbp_value]
#[orm(table = BMBP_RBAC_ORGAN, id = r_id, exclude = organ_children)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrgan {
    organ_type: BmbpOrganType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpOrganType {
    Unit,
    Units,
    Dept,
    Post,
    Person,
}

impl Default for BmbpOrganType {
    fn default() -> Self {
        BmbpOrganType::Unit
    }
}

impl ToString for BmbpOrganType {
    fn to_string(&self) -> String {
        match self {
            BmbpOrganType::Unit => "Unit".to_string(),
            BmbpOrganType::Units => "Units".to_string(),
            BmbpOrganType::Dept => "Dept".to_string(),
            BmbpOrganType::Post => "Post".to_string(),
            BmbpOrganType::Person => "Person".to_string(),
        }
    }
}

impl From<BmbpOrganType> for BmbpValue {
    fn from(value: BmbpOrganType) -> Self {
        BmbpValue::from(value.to_string())
    }
}

impl From<&BmbpOrganType> for BmbpValue {
    fn from(value: &BmbpOrganType) -> Self {
        BmbpValue::from(value.to_string())
    }
}
