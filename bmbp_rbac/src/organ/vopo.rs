use serde::{Deserialize, Serialize};

use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, PageReqVo, TreeNode};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct QueryParam {
    r_id: String,
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_type: String,
}

impl QueryParam {
    pub fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub fn set_organ_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    pub fn set_parent_organ_id(&mut self, parent_organ_id: String) -> &mut Self {
        self.parent_organ_id = parent_organ_id;
        self
    }
    pub fn set_organ_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = organ_title;
        self
    }
    pub fn set_organ_path(&mut self, organ_path: String) -> &mut Self {
        self.organ_path = organ_path;
        self
    }
    pub fn set_organ_type(&mut self, organ_type: String) -> &mut Self {
        self.organ_type = organ_type;
        self
    }
}

// 分页查询参数
pub type PageQueryParam = PageReqVo<QueryParam>;

// 组织树
const BMBP_RBAC_ORGAN: &str = "BMBP_RBAC_ORGAN";
// 单位分组明细
const BMBP_RBAC_ORGAN_UNITS: &str = "BMBP_RBAC_ORGAN_UNITS";
// 单位明细
const BMBP_RBAC_ORGAN_UNIT: &str = "BMBP_RBAC_ORGAN_UNIT";
// 部门明细
const BMBP_RBAC_ORGAN_DEPT: &str = "BMBP_RBAC_ORGAN_DEPT";
// 岗位明细
const BMBP_RBAC_ORGAN_POST: &str = "BMBP_RBAC_ORGAN_POST";
// 人员明细
const BMBP_RBAC_ORGAN_USER: &str = "BMBP_RBAC_ORGAN_USER";
// 虚拟组织树
const BMBP_RBAC_ORGAN_VIRTUAL: &str = "BMBP_RBAC_ORGAN_VIRTUAL";
// 岗位分管明细
const BMBP_RBAC_ORGAN_POST_MANAGER: &str = "BMBP_RBAC_ORGAN_POST_MANAGER";
// 岗位分管明细
const BMBP_RBAC_ORGAN_USER_POST: &str = "BMBP_RBAC_ORGAN_USER_POST";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BmbpOrganType {
    Unit,
    Units,
    Dept,
    Person,
}

impl Default for BmbpOrganType {
    fn default() -> Self {
        BmbpOrganType::Unit
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganVo {
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_data_id: String,
    organ_type: BmbpOrganType,
    children: Vec<BmbpOrganVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl BaseOrmVoPo for BmbpOrganVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo;
    }

    fn vo_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "parent_organ_id".to_string(),
            "organ_title".to_string(),
            "organ_path".to_string(),
            "organ_type".to_string(),
            "organ_data_id".to_string(),
        ]
    }
}

impl BmbpOrganVo {
    pub fn new() -> Self {
        BmbpOrganVo::default()
    }
}

impl TreeNode<BmbpOrganVo> for BmbpOrganVo {
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
    fn children(&self) -> &[BmbpOrganVo] {
        self.children.as_slice()
    }
    fn set_children(&mut self, children: Vec<BmbpOrganVo>) {
        self.children = children
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitsVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganDeptVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPostVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUserVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPostManagerVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUserPostVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganVirtualVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}
