use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use bmbp_orm_ins::BmbpScriptSql;
use bmbp_orm_macro::orm;
use bmbp_orm_macro::{model, page};
use bmbp_types::vo::BaseOrmModel;
use bmbp_types::BmbpValue;
use bmbp_types::{BmbpBaseModel, BmbpPageReqVo, TreeNode};

// 单位分组明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_UNITS: &str = "BMBP_RBAC_ORGAN_UNITS";
// 单位明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_UNIT: &str = "BMBP_RBAC_ORGAN_UNIT";
// 部门明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_DEPT: &str = "BMBP_RBAC_ORGAN_DEPT";
// 岗位明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_POST: &str = "BMBP_RBAC_ORGAN_POST";
// 人员明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_PERSON: &str = "BMBP_RBAC_ORGAN_PERSON";
// 虚拟组织树
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_VIRTUAL: &str = "BMBP_RBAC_ORGAN_VIRTUAL";
// 岗位分管明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_POST_MANAGER: &str = "BMBP_RBAC_ORGAN_POST_MANAGER";
// 岗位分管明细
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN_USER_POST: &str = "BMBP_RBAC_ORGAN_USER_POST";

#[page]
pub struct OrganQueryParam {
    r_id: String,
    parent_organ_id: String,
    organ_id: String,
}

#[orm]
pub struct BmbpRbacOrgan {
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_data_id: String,
    organ_type: BmbpOrganType,
    #[skip]
    children: Vec<BmbpRbacOrgan>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BmbpOrganType {
    Unit = 0,
    Units = 1,
    Dept = 2,
    Post = 3,
    Person = 4,
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
impl Default for BmbpOrganType {
    fn default() -> Self {
        BmbpOrganType::Unit
    }
}

#[allow(dead_code)]
impl TreeNode<BmbpRbacOrgan> for BmbpRbacOrgan {
    fn node_id(&self) -> &String {
        &self.organ_id
    }
    fn node_parent_id(&self) -> &String {
        &self.parent_organ_id
    }
    fn node_title(&self) -> &String {
        &self.organ_title
    }
    fn node_data_id(&self) -> &String {
        &self.organ_data_id
    }
    fn node_path(&self) -> &String {
        &self.organ_path
    }
    fn children(&self) -> &[BmbpRbacOrgan] {
        self.children.as_slice()
    }
    fn set_children(&mut self, children: Vec<BmbpRbacOrgan>) -> &mut Self {
        self.children = children;
        self
    }
}
