use bmbp_orm_ins::BmbpScriptSql;

/// 获取组织机构信息表
fn get_orm_table_name() -> String {
    "BMBP_RBAC_ORGAN".to_string()
}

pub(crate) struct OrganScript();
impl OrganScript {
    /// 组织机构查询语句
    pub(crate) fn query_script() -> BmbpScriptSql {
        let mut query_script = BmbpScriptSql::new();
        query_script.from(get_orm_table_name().as_str());
        query_script
    }
}
