use serde::{Deserialize, Serialize};

use bmbp_orm_macro::{method, model, page, tree};
use bmbp_types::BmbpTree;

#[method]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
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
