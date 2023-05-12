use serde::{Deserialize, Serialize};

use bmbp_orm_macro::{method, model, page, tree};
use bmbp_types::BmbpValue;
use bmbp_types::TreeNode;

#[page]
#[method]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrganQueryParam {
    r_id: String,
    parent_organ_id: String,
    organ_id: String,
}

#[tree(organ)]
#[model]
#[method]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BmbpRbacOrgan {
    organ_type: BmbpOrganType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpOrganType {
    Unit = 0,
    Units = 1,
    Dept = 2,
    Post = 3,
    Person = 4,
}

impl ToString for BmbpOrganType {
    fn to_string(&self) -> String {
        match self {
            BmbpOrganType::Unit => "0".to_string(),
            BmbpOrganType::Units => "1".to_string(),
            BmbpOrganType::Dept => "2".to_string(),
            BmbpOrganType::Post => "3".to_string(),
            BmbpOrganType::Person => "4".to_string(),
        }
    }
}

impl Default for BmbpOrganType {
    fn default() -> Self {
        BmbpOrganType::Unit
    }
}

impl Into<BmbpValue> for BmbpOrganType {
    fn into(self) -> BmbpValue {
        match self {
            BmbpOrganType::Unit => BmbpValue::from(0),
            BmbpOrganType::Units => BmbpValue::from(1),
            BmbpOrganType::Dept => BmbpValue::from(2),
            BmbpOrganType::Post => BmbpValue::from(3),
            BmbpOrganType::Person => BmbpValue::from(4),
        }
    }
}
