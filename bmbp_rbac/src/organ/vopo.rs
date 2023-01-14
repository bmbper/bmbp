use serde::{Deserialize, Serialize};

use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, PageReqVo, TreeNode};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    pub fn get_r_id(&self) -> &String {
        &self.r_id
    }
    pub fn get_organ_id(&self) -> &String {
        &self.organ_id
    }
    pub fn get_parent_organ_id(&self) -> &String {
        &self.parent_organ_id
    }

    pub fn get_organ_title(&self) -> &String {
        &self.organ_title
    }
    pub fn get_organ_path(&self) -> &String {
        &self.organ_path
    }
}

// 分页查询参数
#[allow(dead_code)]
pub type PageQueryParam = PageReqVo<QueryParam>;

// 组织树
#[allow(dead_code)]
pub const BMBP_RBAC_ORGAN: &str = "BMBP_RBAC_ORGAN";
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
            BmbpOrganType::Unit => "unit".to_string(),
            BmbpOrganType::Units => "units".to_string(),
            BmbpOrganType::Dept => "dept".to_string(),
            BmbpOrganType::Post => "post".to_string(),
            BmbpOrganType::Person => "person".to_string(),
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
impl BmbpOrganVo {
    pub fn new() -> Self {
        BmbpOrganVo::default()
    }

    pub fn set_organ_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    pub fn set_parent_organ_id(&mut self, parent_organ_id: String) -> &mut Self {
        self.parent_organ_id = parent_organ_id;
        self
    }
    pub fn set_organ_path(&mut self, organ_path: String) -> &mut Self {
        self.organ_path = organ_path;
        self
    }

    pub fn set_organ_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = organ_title;
        self
    }

    pub fn set_organ_data_id(&mut self, organ_data_id: String) -> &mut Self {
        self.organ_data_id = organ_data_id;
        self
    }

    pub fn set_organ_type(&mut self, organ_type: BmbpOrganType) -> &mut Self {
        self.organ_type = organ_type;
        self
    }

    pub fn get_organ_id(&self) -> &String {
        &self.organ_id
    }
    pub fn get_organ_data_id(&self) -> &String {
        &self.organ_data_id
    }
    pub fn get_organ_title(&self) -> &String {
        &self.organ_title
    }
    pub fn get_organ_path(&self) -> &String {
        &self.organ_path
    }
    pub fn get_parent_organ_id(&self) -> &String {
        &self.parent_organ_id
    }
    pub fn get_organ_type(&self) -> &BmbpOrganType {
        &self.organ_type
    }
}
#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitsVo {
    organ_id: String,
    units_desc: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BmbpOrganUnitsVo {
    pub fn new() -> Self {
        BmbpOrganUnitsVo::default()
    }
}

#[allow(dead_code)]
impl BmbpOrganUnitsVo {
    pub fn organ_id(&self) -> &String {
        &self.organ_id
    }
    pub fn organ_desc(&self) -> &String {
        &self.units_desc
    }
    pub fn set_organ_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    pub fn set_organ_desc(&mut self, units_desc: String) -> &mut Self {
        self.units_desc = units_desc;
        self
    }
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganUnitsVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec!["organ_id".to_string(), "units_desc".to_string()]
    }
}
#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitVo {
    organ_id: String,
    // 所属行业
    industry_code: String,
    // 所属领域：
    domain_code: String,
    // 单位性质
    unit_type: String,
    // 行政级別
    admin_grade: String,
    // 联系人
    concat_person: String,
    // 联系电话
    concat_phone: String,
    // 联系邮件
    concat_email: String,
    // 联系手机号
    concat_tel: String,
    // 联系地址
    concat_address: String,
    // 省
    province_code: String,
    // 市
    city_code: String,
    // 区
    district_code: String,

    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganUnitVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitReportVo {
    organ_id: String,
    // 上报单位
    report_organ_id: String,
    // 省
    report_province_code: String,
    // 市
    report_city_code: String,
    // 区
    report_district_code: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BmbpOrganUnitReportVo {
    pub fn new() -> Self {
        BmbpOrganUnitReportVo::default()
    }
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganUnitReportVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "report_organ_id".to_string(),
            "report_province_code".to_string(),
            "report_city_code".to_string(),
            "report_district_code".to_string(),
        ]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganDeptVo {
    organ_id: String,
    // 部门编制
    dept_head_count: usize,
    // 部门说明
    dept_desc: String,
    //
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganDeptVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPostVo {
    organ_id: String,
    // 岗位类型
    post_type: String,
    // 岗位描述
    post_desc: String,
    // 联系人
    concat_person: String,
    // 联系电话
    concat_phone: String,
    // 联系邮件
    concat_email: String,
    // 联系手机号
    concat_tel: String,
    // 联系地址
    concat_address: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganPostVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPersonVo {
    organ_id: String,
    //性別
    gender: String,
    //民族
    nation: String,
    // 政治面貌
    politics: String,
    //职称
    positional_title: String,
    // 职级
    rank: String,
    // 出生日期
    birth_day: String,
    // 年龄
    age: usize,
    // 手机号
    tel: String,
    // 身份证号
    id_no: String,
    // 学历
    education: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganPersonVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPostManagerVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl BaseOrmVoPo for BmbpOrganPostManagerVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganPersonPostVo {
    organ_id: String,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[allow(dead_code)]
impl BaseOrmVoPo for BmbpOrganPersonPostVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
    }

    fn vo_fields() -> Vec<String> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganVirtualVo {
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_data_id: String,
    organ_type: BmbpOrganType,
    children: Vec<BmbpOrganVirtualVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl TreeNode<BmbpOrganVirtualVo> for BmbpOrganVirtualVo {
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
    fn children(&self) -> &[BmbpOrganVirtualVo] {
        self.children.as_slice()
    }
    fn set_children(&mut self, children: Vec<BmbpOrganVirtualVo>) {
        self.children = children
    }
}

impl BaseOrmVoPo for BmbpOrganVirtualVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo
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
