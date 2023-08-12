use crate::app::dao::RbacAppDao;
use bmbp_app_common::{
    BmbpHashMap, BmbpResp, BmbpValue, FieldValidRule, PageParams, PageVo, ValidRule, ValidType,
};
use bmbp_app_utils::{
    add_insert_default_value, add_update_default_value, is_empty_prop, valid_field_rule_slice,
};

use super::script::RbacAppScript;

pub struct RbacAppService;
impl RbacAppService {
    pub(crate) async fn find_page(
        params: &PageParams<BmbpHashMap>,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let query_script = RbacAppScript::query_script();
        let query_params = BmbpHashMap::new();
        if let Some(_) = params.get_params() {}
        let page_vo = RbacAppDao::find_page(
            &query_script.to_script(),
            &query_params,
            params.get_page_no(),
            params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }

    pub(crate) async fn find_list(_params: &BmbpHashMap) -> BmbpResp<Option<Vec<BmbpHashMap>>> {
        let query_script = RbacAppScript::query_script();
        let query_params = BmbpHashMap::new();
        let page_vo = RbacAppDao::find_list(&query_script.to_script(), &query_params).await?;
        Ok(page_vo)
    }

    pub(crate) async fn find_info(record_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut params = BmbpHashMap::new();
        params.insert("recordId".to_string(), BmbpValue::from(record_id));
        let mut query_script = RbacAppScript::query_script();
        query_script.filter("record_id = #{recordId}");
        let app_info = RbacAppDao::find_info(&query_script.to_script(), &params).await?;
        Ok(app_info)
    }

    /// update_status 更新记录状态
    pub(crate) async fn update_status(record_id: &String, status: String) -> BmbpResp<usize> {
        let mut params = BmbpHashMap::new();
        params.insert("recordId".to_string(), BmbpValue::from(record_id));
        params.insert("recordStatus".to_string(), BmbpValue::from(status));
        let updage_script = RbacAppScript::update_script_for_status();
        let app_info = RbacAppDao::update_status(&updage_script.to_script(), &params).await?;
        Ok(app_info)
    }

    pub(crate) async fn delete_app(record_id: &String) -> BmbpResp<usize> {
        let mut params = BmbpHashMap::new();
        params.insert("recordId".to_string(), BmbpValue::from(record_id));
        let updage_script = RbacAppScript::delete_script_by_reocrd_id();
        let row_count = RbacAppDao::delete_app(&updage_script.to_script(), &params).await?;
        Ok(row_count)
    }

    pub(crate) async fn save_app(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_app(params).await
        } else {
            Self::update_app(params).await
        }
    }
    pub(crate) async fn insert_app(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if let Some(err) = Self::valid_insert_app_data(params) {
            return Err(err);
        }
        add_insert_default_value(params);
        let script = RbacAppScript::insert_script();
        RbacAppDao::insert_app(&script.to_script(), params).await
    }
    pub(crate) async fn update_app(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        add_update_default_value(params);
        RbacAppDao::update_app(&"".to_string(), params).await
    }

    /// 验证应用新增数据
    fn valid_insert_app_data(
        params: &mut std::collections::HashMap<String, BmbpValue>,
    ) -> Option<bmbp_app_common::BmbpError> {
        let valid_rule = vec![
            FieldValidRule(
                "appCode".to_string(),
                ValidRule(ValidType::NotEmpty, "应用编码不能为空!".to_string()),
            ),
            FieldValidRule(
                "appTitle".to_string(),
                ValidRule(ValidType::NotEmpty, "应用标题不能为空!".to_string()),
            ),
            FieldValidRule(
                "appKey".to_string(),
                ValidRule(ValidType::NotEmpty, "应用标识不能为空!".to_string()),
            ),
            FieldValidRule(
                "appSecrectKey".to_string(),
                ValidRule(ValidType::NotEmpty, "应用密钥不能为空!".to_string()),
            ),
        ];
        valid_field_rule_slice(params, valid_rule.as_slice())
    }
}
