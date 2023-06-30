use std::string::ToString;

use serde::{Deserialize, Serialize};

use bmbp_app_common::{BmbpBaseModel, BmbpTree};
use bmbp_app_common::{BmbpHashMap, BmbpValue};

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrganQueryParam {
    record_id: Option<String>,
    organ_parent_code: Option<String>,
    organ_title: Option<String>,
}
#[allow(dead_code)]
impl OrganQueryParam {
    pub fn new() -> Self {
        OrganQueryParam::default()
    }
    pub fn set_record_id(&mut self, record_id: String) -> &mut Self {
        self.record_id = Some(record_id);
        self
    }
    pub fn set_organ_parent_code(&mut self, parent_organ_code: String) -> &mut Self {
        self.organ_parent_code = Some(parent_organ_code);
        self
    }

    pub fn set_organ_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = Some(organ_title);
        self
    }
    pub fn get_record_id(&self) -> Option<&String> {
        self.record_id.as_ref()
    }
    pub fn get_organ_parent_code(&self) -> Option<&String> {
        self.organ_parent_code.as_ref()
    }
    pub fn get_organ_title(&self) -> Option<&String> {
        self.organ_title.as_ref()
    }
}

/// 组织机构
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrgan {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 组织编码
    organ_code: Option<String>,
    /// 上级组织编码
    organ_parent_code: Option<String>,
    /// 组织名称
    organ_title: Option<String>,
    /// 组织编码路径
    organ_code_path: Option<String>,
    /// 组织名称路径
    organ_title_path: Option<String>,
    /// 惟一数据标识
    organ_data_id: Option<String>,
    /// 组织类型
    organ_type: BmbpRbacOrganType,
    /// 下级组织
    organ_children: Option<Vec<BmbpRbacOrgan>>,
}
#[allow(dead_code)]
impl BmbpRbacOrgan {
    pub fn new() -> Self {
        BmbpRbacOrgan::default()
    }

    pub fn set_base_model(&mut self, base_model: BmbpBaseModel) -> &mut Self {
        self.base = base_model;
        self
    }
    pub fn set_organ_code(&mut self, organ_code: String) -> &mut Self {
        self.organ_code = Some(organ_code);
        self
    }
    pub fn set_organ_parent_code(&mut self, organ_parent_code: String) -> &mut Self {
        self.organ_parent_code = Some(organ_parent_code);
        self
    }
    pub fn set_organ_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = Some(organ_title);
        self
    }
    pub fn set_organ_code_path(&mut self, organ_code_path: String) -> &mut Self {
        self.organ_code_path = Some(organ_code_path);
        self
    }
    pub fn set_organ_title_path(&mut self, organ_title_path: String) -> &mut Self {
        self.organ_title_path = Some(organ_title_path);
        self
    }
    pub fn set_organ_data_id(&mut self, organ_data_id: String) -> &mut Self {
        self.organ_data_id = Some(organ_data_id);
        self
    }
    pub fn set_organ_children(&mut self, organ_children: Vec<BmbpRbacOrgan>) -> &mut Self {
        self.organ_children = Some(organ_children);
        self
    }
    pub fn set_organ_type(&mut self, organ_type: BmbpRbacOrganType) -> &mut Self {
        self.organ_type = organ_type;
        self
    }

    pub fn get_mut_base_model(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }
    pub fn get_base_model(&self) -> &BmbpBaseModel {
        &self.base
    }
    pub fn get_organ_code(&self) -> Option<&String> {
        self.organ_code.as_ref()
    }
    pub fn get_organ_parent_code(&self) -> Option<&String> {
        self.organ_parent_code.as_ref()
    }
    pub fn get_organ_title(&self) -> Option<&String> {
        self.organ_title.as_ref()
    }
    pub fn get_organ_code_path(&self) -> Option<&String> {
        self.organ_code_path.as_ref()
    }
    pub fn get_organ_title_path(&self) -> Option<&String> {
        self.organ_title_path.as_ref()
    }
    pub fn get_organ_data_id(&self) -> Option<&String> {
        self.organ_data_id.as_ref()
    }
    pub fn get_organ_children(&self) -> Option<&Vec<BmbpRbacOrgan>> {
        self.organ_children.as_ref()
    }
    pub fn get_mut_organ_children(&mut self) -> Option<&mut Vec<BmbpRbacOrgan>> {
        self.organ_children.as_mut()
    }
    pub fn get_organ_type(&self) -> &BmbpRbacOrganType {
        &self.organ_type
    }
}

impl BmbpTree<BmbpRbacOrgan> for BmbpRbacOrgan {
    fn get_tree_code(&self) -> Option<&String> {
        self.get_organ_code()
    }

    fn get_tree_code_path(&self) -> Option<&String> {
        self.get_organ_code_path()
    }

    fn get_tree_parent_code(&self) -> Option<&String> {
        self.get_organ_parent_code()
    }

    fn get_tree_data_id(&self) -> Option<&String> {
        self.get_organ_data_id()
    }

    fn get_tree_title(&self) -> Option<&String> {
        self.get_organ_title()
    }

    fn get_tree_title_path(&self) -> Option<&String> {
        self.get_organ_title_path()
    }

    fn get_tree_children(&self) -> Option<&Vec<BmbpRbacOrgan>> {
        self.get_organ_children()
    }

    fn get_mut_tree_children(&mut self) -> Option<&mut Vec<BmbpRbacOrgan>> {
        self.get_mut_organ_children()
    }

    fn set_tree_code(&mut self, tree_code: String) -> &mut Self {
        self.set_organ_code(tree_code)
    }

    fn set_tree_parent_code(&mut self, tree_parent_code: String) -> &mut Self {
        self.set_organ_parent_code(tree_parent_code)
    }

    fn set_tree_data_id(&mut self, tree_data_id: String) -> &mut Self {
        self.set_organ_data_id(tree_data_id)
    }

    fn set_tree_title(&mut self, tree_title: String) -> &mut Self {
        self.set_organ_title(tree_title)
    }

    fn set_tree_code_path(&mut self, tree_code_path: String) -> &mut Self {
        self.set_organ_code_path(tree_code_path)
    }

    fn set_tree_title_path(&mut self, tree_title_path: String) -> &mut Self {
        self.set_organ_title_path(tree_title_path)
    }

    fn set_tree_children(&mut self, children: Vec<BmbpRbacOrgan>) -> &mut Self {
        self.set_organ_children(children)
    }
}

impl From<&BmbpRbacOrgan> for BmbpValue {
    fn from(value: &BmbpRbacOrgan) -> Self {
        let hasp_map = BmbpHashMap::from(value);
        BmbpValue::Map(hasp_map)
    }
}

impl From<&BmbpRbacOrgan> for BmbpHashMap {
    fn from(value: &BmbpRbacOrgan) -> Self {
        let mut hash_map = BmbpHashMap::new();
        let bash_map = BmbpHashMap::from(value.get_base_model());
        hash_map.extend(bash_map);
        hash_map.insert(
            "organ_code".to_string(),
            BmbpValue::from(value.get_organ_code()),
        );
        hash_map.insert(
            "organ_parent_code".to_string(),
            BmbpValue::from(value.get_organ_parent_code()),
        );
        hash_map.insert(
            "organ_title".to_string(),
            BmbpValue::from(value.get_organ_title()),
        );
        hash_map.insert(
            "organ_code_path".to_string(),
            BmbpValue::from(value.get_organ_code_path()),
        );
        hash_map.insert(
            "organ_title_path".to_string(),
            BmbpValue::from(value.get_organ_title_path()),
        );
        hash_map.insert(
            "organ_data_id".to_string(),
            BmbpValue::from(value.get_organ_data_id()),
        );
        hash_map.insert(
            "organ_type".to_string(),
            BmbpValue::from(value.get_organ_type()),
        );

        hash_map
    }
}
impl From<&mut BmbpRbacOrgan> for BmbpHashMap {
    fn from(value: &mut BmbpRbacOrgan) -> Self {
        let mut hash_map = BmbpHashMap::new();
        let bash_map = BmbpHashMap::from(value.get_base_model());
        hash_map.extend(bash_map);
        hash_map.insert(
            "organ_code".to_string(),
            BmbpValue::from(value.get_organ_code()),
        );
        hash_map.insert(
            "organ_parent_code".to_string(),
            BmbpValue::from(value.get_organ_parent_code()),
        );
        hash_map.insert(
            "organ_title".to_string(),
            BmbpValue::from(value.get_organ_title()),
        );
        hash_map.insert(
            "organ_code_path".to_string(),
            BmbpValue::from(value.get_organ_code_path()),
        );
        hash_map.insert(
            "organ_title_path".to_string(),
            BmbpValue::from(value.get_organ_title_path()),
        );
        hash_map.insert(
            "organ_data_id".to_string(),
            BmbpValue::from(value.get_organ_data_id()),
        );
        hash_map.insert(
            "organ_type".to_string(),
            BmbpValue::from(value.get_organ_type()),
        );

        hash_map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRbacOrganType {
    /// 单位
    UNIT,
    /// 分组
    UNITS,
    /// 部门
    DEPT,
    /// 岗位
    POST,
    /// 人员
    PERSON,
}

impl Default for BmbpRbacOrganType {
    fn default() -> Self {
        BmbpRbacOrganType::UNIT
    }
}

impl ToString for BmbpRbacOrganType {
    fn to_string(&self) -> String {
        match self {
            BmbpRbacOrganType::UNIT => "UNIT".to_string(),
            BmbpRbacOrganType::UNITS => "UNITS".to_string(),
            BmbpRbacOrganType::DEPT => "DEPT".to_string(),
            BmbpRbacOrganType::POST => "POST".to_string(),
            BmbpRbacOrganType::PERSON => "PERSON".to_string(),
        }
    }
}

impl From<BmbpRbacOrganType> for BmbpValue {
    fn from(value: BmbpRbacOrganType) -> Self {
        BmbpValue::from(value.to_string())
    }
}

impl From<&BmbpRbacOrganType> for BmbpValue {
    fn from(value: &BmbpRbacOrganType) -> Self {
        BmbpValue::from(value.to_string())
    }
}
