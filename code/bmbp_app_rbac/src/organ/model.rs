use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use bmbp_rdbc_orm::{BmbpOrmRdbcTree, RdbcModel, RdbcOrmRow};

use crate::organ::model::BmbpRbacOrganInfo::Group;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrganQueryParams {
    data_id: Option<String>,
    code: Option<String>,
    parent_code: Option<String>,
    show_level: Option<usize>,
}

impl OrganQueryParams {
    pub fn new() -> Self {
        OrganQueryParams {
            data_id: None,
            code: None,
            parent_code: None,
            show_level: None,
        }
    }
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn set_data_id_option(&mut self, data_id: Option<String>) -> &mut Self {
        self.data_id = data_id;
        self
    }
    pub fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }
    pub fn set_code(&mut self, code: String) -> &mut Self {
        self.code = Some(code);
        self
    }
    pub fn set_code_option(&mut self, code: Option<String>) -> &mut Self {
        self.code = code;
        self
    }
    pub fn get_parent_code(&self) -> Option<&String> {
        self.parent_code.as_ref()
    }
    pub fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = Some(parent_code);
        self
    }
    pub fn set_parent_code_option(&mut self, parent_code: Option<String>) -> &mut Self {
        self.parent_code = parent_code;
        self
    }
    pub fn get_show_level(&self) -> Option<&usize> {
        self.show_level.as_ref()
    }
    pub fn set_show_level(&mut self, show_level: usize) -> &mut Self {
        self.show_level = Some(show_level);
        self
    }
    pub fn set_show_level_option(&mut self, show_level: Option<usize>) -> &mut Self {
        self.show_level = show_level;
        self
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrgan {
    organ_type: Option<BmbpRdbcOrganType>,
    organ_info: Option<BmbpRbacOrganInfo>,
}

impl BmbpRbacOrgan {
    pub fn new() -> Self {
        BmbpRbacOrgan {
            organ_type: None,
            organ_info: None,
        }
    }
    pub fn get_organ_type(&self) -> Option<&BmbpRdbcOrganType> {
        self.organ_type.as_ref()
    }
    pub fn set_organ_type(&mut self, organ_type: BmbpRdbcOrganType) -> &mut Self {
        self.organ_type = Some(organ_type);
        self
    }
    pub fn set_organ_type_option(&mut self, organ_type: Option<BmbpRdbcOrganType>) -> &mut Self {
        self.organ_type = organ_type;
        self
    }
    pub fn get_organ_info(&self) -> Option<&BmbpRbacOrganInfo> {
        self.organ_info.as_ref()
    }
    pub fn set_organ_info(&mut self, organ_info: BmbpRbacOrganInfo) -> &mut Self {
        self.organ_info = Some(organ_info);
        self
    }
}
pub type BmbpRbacOrganTree = BmbpOrmRdbcTree<BmbpRbacOrgan>;
impl From<RdbcOrmRow> for BmbpRbacOrgan {
    fn from(row: RdbcOrmRow) -> Self {
        let mut organ = BmbpRbacOrgan::default();
        if let Some(data) = row.get_data().get("organ_type") {
            organ.set_organ_type_option(BmbpRdbcOrganType::value_of(data.to_string()));
        }
        organ
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BmbpRbacOrganInfo {
    Region(BmbpRbacOrganRegion),
    Group(BmbpRbacOrganGroup),
    Unit(BmbpRbacOrganUnit),
    Dept(BmbpRbacOrganDept),
    Post(BmbpRbacOrganPost),
    Person(BmbpRbacOrganPerson),
}

impl Default for BmbpRbacOrganInfo {
    fn default() -> Self {
        Group(BmbpRbacOrganGroup::default())
    }
}

impl RdbcModel for BmbpRbacOrgan {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN".to_string()
    }
    fn get_table_fields() -> Vec<String> {
        vec!["organ_type".to_string()]
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[serde(untagged)]
#[repr(u16)]
pub enum BmbpRdbcOrganType {
    REGION = 0,
    GROUP,
    UNIT,
    DEPT,
    POST,
    PERSON,
}

impl BmbpRdbcOrganType {
    pub fn value_of(data: String) -> Option<Self> {
        match data.to_uppercase().as_str() {
            "0" => Some(BmbpRdbcOrganType::REGION),
            "1" => Some(BmbpRdbcOrganType::GROUP),
            "2" => Some(BmbpRdbcOrganType::UNIT),
            "3" => Some(BmbpRdbcOrganType::DEPT),
            "4" => Some(BmbpRdbcOrganType::POST),
            "5" => Some(BmbpRdbcOrganType::PERSON),
            _ => Some(BmbpRdbcOrganType::GROUP),
        }
    }
    pub fn value(&self) -> u16 {
        match self {
            BmbpRdbcOrganType::REGION => 0,
            BmbpRdbcOrganType::GROUP => 1,
            BmbpRdbcOrganType::UNIT => 2,
            BmbpRdbcOrganType::DEPT => 3,
            BmbpRdbcOrganType::POST => 4,
            BmbpRdbcOrganType::PERSON => 5,
        }
    }
}
impl Default for BmbpRdbcOrganType {
    fn default() -> Self {
        BmbpRdbcOrganType::GROUP
    }
}
impl Display for BmbpRdbcOrganType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganRegion {
    organ_id: Option<String>,
    region_code: Option<String>,
}
impl RdbcModel for BmbpRbacOrganRegion {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_REGION".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "region_code".to_string(),
            "region_name".to_string(),
        ]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganGroup {
    organ_id: Option<String>,
    group_desc: Option<String>,
}

impl RdbcModel for BmbpRbacOrganGroup {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_GROUP".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec!["organ_id".to_string(), "group_desc".to_string()]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganUnit {
    organ_id: Option<String>,
    unit_desc: Option<String>,
    /// 法人
    monitor: Option<String>,
    /// 统一社会信息用代码
    unified_code: Option<String>,
    /// 企业性质
    unit_type: Option<String>,
    /// 联系电话
    unit_tel: Option<String>,
    /// 企业地址 省市区
    unit_region: Option<String>,
    /// 企业地址 街道
    unit_address: Option<String>,
    /// 邮政编码
    unit_postcode: Option<String>,
}

impl RdbcModel for BmbpRbacOrganUnit {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_UNIT".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "unit_desc".to_string(),
            "monitor".to_string(),
            "unified_code".to_string(),
            "unit_type".to_string(),
            "unit_tel".to_string(),
            "unit_region".to_string(),
            "unit_address".to_string(),
            "unit_postcode".to_string(),
        ]
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganDept {
    organ_id: Option<String>,
    /// 部门负责人
    monitor: Option<String>,
    /// 部门职责
    dept_desc: Option<String>,
    /// 部门电话
    dept_phone: Option<String>,
}
impl RdbcModel for BmbpRbacOrganDept {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_DEPT".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "monitor".to_string(),
            "dept_desc".to_string(),
            "dept_phone".to_string(),
        ]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganPost {
    organ_id: Option<String>,
    /// 岗位职责
    post_duty: Option<String>,
}
impl RdbcModel for BmbpRbacOrganPost {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_POST".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec!["organ_id".to_string(), "post_duty".to_string()]
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacOrganPerson {
    organ_id: Option<String>,
    /// 名称
    name: Option<String>,
    /// 性别
    gender: Option<String>,
    /// 身份证号码
    id_card: Option<String>,
    /// 手机号码
    phone: Option<String>,
    /// 邮箱
    email: Option<String>,
    /// 籍贯
    native_place: Option<String>,
    /// 民族
    nation: Option<String>,
    /// 政治面貌
    political: Option<String>,
    /// 学历
    education: Option<String>,
    /// 学位
    academic: Option<String>,
}
impl RdbcModel for BmbpRbacOrganPerson {
    fn get_table_name() -> String {
        "BMBP_RBAC_ORGAN_PERSON".to_string()
    }

    fn get_table_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "name".to_string(),
            "gender".to_string(),
            "id_card".to_string(),
            "phone".to_string(),
            "email".to_string(),
            "native_place".to_string(),
            "nation".to_string(),
            "political".to_string(),
            "education".to_string(),
            "academic".to_string(),
        ]
    }
}
