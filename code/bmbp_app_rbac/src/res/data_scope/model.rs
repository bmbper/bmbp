use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacDataScope {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 权限编码
    scope_code: Option<String>,
    /// 权限名称
    scope_title: Option<String>,
    /// 权限类型
    scope_type: BmbpRbacDataScopeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BmbpRbacDataScopeType {
    /// 同创建人
    CreateUser,
    /// 同创建岗位
    CreatePost,
    /// 同创建部门
    CreateDept,
    /// 同创建单位
    CreateUnit,
    /// 同创建分组
    CreateUnits,
    /// 同角色
    CreateRole,
    /// 指定人
    AssignUser,
    /// 指定创建岗位
    AssignPost,
    /// 指定创建部门
    AssignDept,
    /// 指定创建单位
    AssignUnit,
    /// 指定创建分组
    AssignUnits,
    /// 指定角色
    AssignRole,
    /// 指定Scope, 填写SQL查询条件
    AssingScope,
    /// 指定实现, 填写实现类名称
    AssingImpl,
}
impl Default for BmbpRbacDataScopeType {
    fn default() -> Self {
        BmbpRbacDataScopeType::CreateUser
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacDataScopeOrgInfo {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 数据权限编码
    scope_code: Option<String>,
    /// 组织编码
    organ_code: Option<String>,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRbacDataScopeScriptInfo {
    /// 公共信息
    #[serde(flatten)]
    base: BmbpBaseModel,
    /// 数据权限编码
    scope_code: Option<String>,
    /// SQL语句、自定义名称
    scope_value: Option<String>,
}
