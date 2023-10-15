use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, FieldValidRule, PageParams, PageVo, ValidRule,
    ValidType, DEFAULT_PASSWORD,
};
use bmbp_app_curd::{CurdDao, CurdScript};
use bmbp_app_utils::{
    add_insert_default_value, add_update_default_value, is_empty_prop, sha2_256_encode_upper,
    valid_field_rule, valid_field_rule_slice,
};

use crate::organ::OrganService;

use super::{dao::UserDao, script::UserScript};

pub struct UserService;

impl UserService {
    /// 分页查询组织列表
    pub async fn find_user_page(
        page_params: &PageParams<BmbpHashMap>,
    ) -> BmbpResp<PageVo<BmbpHashMap>> {
        let mut query_script = UserScript::query_script_by_organ();
        let mut query_params = BmbpHashMap::new();
        if let Some(user_params) = page_params.get_params() {
            if !is_empty_prop(user_params, "organId") {
                let organ_record_id = user_params.get("organId").unwrap().to_string();
                if let Some(organ) = OrganService::find_organ_by_id(&organ_record_id).await? {
                    if let Some(organ_code_path) = organ.get("organCodePath") {
                        query_script
                            .filter("org.organ_code_path like CONCAT(#{organCodePath}::TEXT,'%')");
                        query_params.insert("organCodePath".to_string(), organ_code_path.clone());
                    }
                } else {
                    return Err(BmbpError::service("指定的组织不存在！"));
                }
            }
        }
        let page_vo = UserDao::find_page(
            &query_script.to_script(),
            &query_params,
            page_params.get_page_no(),
            page_params.get_page_size(),
        )
        .await?;
        Ok(page_vo)
    }
    /// 查询组织详情-通过R_ID
    pub async fn find_user_by_id(r_id: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let script = UserScript::query_info_script();
        let mut script_params = BmbpHashMap::new();
        script_params.insert("recordId".to_string(), BmbpValue::from(r_id));
        UserDao::find_info(&script.to_script(), &script_params).await
    }
    /// 查询组织详情-通过ORGAN-CODE
    pub async fn find_user_by_user_name(user_name: &String) -> BmbpResp<Option<BmbpHashMap>> {
        let mut script = UserScript::query_script();
        script.filter("user_name = #{userName}");
        let mut script_params = BmbpHashMap::new();
        script_params.insert("userName".to_string(), BmbpValue::from(user_name));
        UserDao::find_info(&script.to_script(), &script_params).await
    }
    /// save_user 保存组织
    pub(crate) async fn save_user(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if is_empty_prop(params, "recordId") {
            Self::insert_user(params).await
        } else {
            Self::update_user(params).await
        }
    }

    /// 插入组织
    pub(crate) async fn insert_user(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        if let Some(err) = Self::valid_insert_user_data(params) {
            return Err(err);
        }

        add_insert_default_value(params);

        if Self::has_same_name_user(params, false).await? {
            return Err(BmbpError::service("已存在相同名称的用户"));
        }

        if Self::has_same_record_id_user(params).await? {
            return Err(BmbpError::service("已存在相同主键的用户"));
        }

        // 密码二次加密
        let password = params.get("userPassword").unwrap().to_string();
        let encode_password = sha2_256_encode_upper(password);
        params.insert(
            "user_password".to_string(),
            BmbpValue::from(encode_password),
        );
        let script = UserScript::insert_script();
        UserDao::insert(&script.to_script(), params).await
    }
    pub(crate) async fn update_user(params: &mut BmbpHashMap) -> BmbpResp<usize> {
        let record_id_valid_rule = FieldValidRule(
            "recordId".to_string(),
            ValidRule(ValidType::NotEmpty, "主键不允许为空".to_string()),
        );
        if let Some(err) = valid_field_rule(params, &record_id_valid_rule) {
            return Err(err);
        }

        if Self::has_same_name_user(params, true).await? {
            return Err(BmbpError::service("已存在相同名称的用户"));
        }

        let mut script = UserScript::update_script();
        add_update_default_value(params);
        if !is_empty_prop(params, "recordNum") {
            script.set_value("record_num", "#{recordNum}");
        }

        if !is_empty_prop(params, "userName") {
            script.set_value("user_name", "#{userName}");
        }

        if !is_empty_prop(params, "userNickName") {
            script.set_value("user_nick_name", "#{userNickName}");
        }

        script.filter("record_id = #{recordId}");
        UserDao::update(&script.to_script(), params).await
    }

    /// 验证应用新增数据
    fn valid_insert_user_data(
        params: &mut std::collections::HashMap<String, BmbpValue>,
    ) -> Option<bmbp_app_common::BmbpError> {
        let valid_rule = vec![
            FieldValidRule(
                "organId".to_string(),
                ValidRule(ValidType::NotEmpty, "用户所属组织不能为空".to_string()),
            ),
            FieldValidRule(
                "userName".to_string(),
                ValidRule(ValidType::NotEmpty, "用户名称不能为空!".to_string()),
            ),
            FieldValidRule(
                "userNickName".to_string(),
                ValidRule(ValidType::NotEmpty, "用户昵称不能为空".to_string()),
            ),
            FieldValidRule(
                "userPassword".to_string(),
                ValidRule(ValidType::NotEmpty, "用户密码不能为空".to_string()),
            ),
        ];
        valid_field_rule_slice(params, valid_rule.as_slice())
    }

    /// 更新组织状态
    pub async fn update_user_status(id: &String, status: &String) -> BmbpResp<usize> {
        let script = UserScript::update_status_script();
        let mut script_params = BmbpHashMap::new();
        add_update_default_value(&mut script_params);
        script_params.insert("recordStatus".to_string(), BmbpValue::from(status));
        script_params.insert("recordId".to_string(), BmbpValue::from(id));
        UserDao::update(&script.to_script(), &script_params).await
    }
    /// 更新组织上级
    pub async fn update_user_organ_by_record_id(
        record_id: &String,
        organ_id: &String,
    ) -> BmbpResp<usize> {
        let current_user = Self::find_user_by_id(record_id).await?;
        if current_user.is_none() {
            return Err(BmbpError::service("待变更的用户不存在"));
        };
        let target_organ = OrganService::find_organ_by_id(organ_id).await?;
        if target_organ.is_none() {
            return Err(BmbpError::service("待变更的组织不存在"));
        }
        let update_script = UserScript::change_organ_script();
        let mut script_params = BmbpHashMap::new();
        add_update_default_value(&mut script_params);
        script_params.insert("organId".to_string(), BmbpValue::from(organ_id));
        script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        let row_count = UserDao::update(&update_script.to_script(), &script_params).await?;
        Ok(row_count)
    }
    /// 删除组织
    pub async fn remove_user_by_id(id: &String) -> BmbpResp<usize> {
        if let Some(user) = Self::find_user_by_id(id).await? {
            if Self::current_user_has_data(&user).await? {
                return Err(BmbpError::service("指定用户织存在业务数据，无法删除"));
            }
            let script = UserScript::delete_script_by_id();
            let mut script_params = BmbpHashMap::new();
            script_params.insert("recordId".to_string(), BmbpValue::from(id));
            UserDao::delete(&script.to_script(), &script_params).await
        } else {
            Err(BmbpError::service("指定的用户不存在，无法删除!"))
        }
    }

    pub(crate) async fn reset_user_password(record_id: &String) -> BmbpResp<usize> {
        let script = UserScript::reset_password_script();
        let mut script_params = BmbpHashMap::new();
        add_update_default_value(&mut script_params);
        script_params.insert("recordId".to_string(), BmbpValue::from(record_id));
        script_params.insert("password".to_string(), BmbpValue::from(DEFAULT_PASSWORD));
        UserDao::update(&script.to_script(), &script_params).await
    }
}

/// 校验逻辑
impl UserService {
    pub async fn has_same_record_id_user(_user: &mut BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
    /// 判断是否包含相同组织
    pub async fn has_same_name_user(user: &mut BmbpHashMap, is_update: bool) -> BmbpResp<bool> {
        if is_empty_prop(user, "userName") {
            return Err(BmbpError::service("用户名称不允许为空"));
        }
        let mut script = UserScript::query_script();
        script.filter("user_name = #{userName}");
        if is_update {
            if is_empty_prop(user, "recordId") {
                return Err(BmbpError::service("请指定待更新的用户"));
            }
            script.filter("record_id != #{recordId}");
        }
        let user_rs = UserDao::find_info(&script.to_script(), &user).await?;
        Ok(user_rs.is_some())
    }
    /// 判断是否关联业务
    pub async fn current_user_has_data(_user: &BmbpHashMap) -> BmbpResp<bool> {
        Ok(false)
    }
}
