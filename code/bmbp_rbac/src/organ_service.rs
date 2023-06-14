use std::collections::HashMap;

use bmbp_orm_ins::{BmbpORM, BmbpScriptSql};
use bmbp_types::{
    BmbpError, BmbpHashMap, BmbpResp, BmbpValue, PageParams, PageRespVo, ROOT_TREE_NODE,
};
use bmbp_util::{insert_decorate, uuid_upper, TreeBuilder};

use crate::organ_model::{BmbpRbacOrgan, OrganQueryParam};

/// 服务声明
pub struct OrganService();

/// CURD 逻辑
impl OrganService {
    /// 查询组织树
    pub async fn find_organ_tree(params: &OrganQueryParam) -> BmbpResp<Vec<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::query_script();
        let mut path_id = "".to_string();

        // 记录ID不为空
        if !params.get_r_id().is_empty() {
            let organ = Self::find_organ_by_id(params.get_r_id()).await?;
            if organ.is_some() {
                path_id = organ.unwrap().get_organ_id().to_string();
            } else {
                return Err(BmbpError::api("指定的节点不存在".to_string()));
            }
        } else if !params.get_organ_id().is_empty() {
            path_id = params.get_organ_id().to_string();
        } else if !params.get_organ_parent_id().is_empty() {
            path_id = params.get_organ_parent_id().to_string();
        }

        if !path_id.is_empty() {
            script_param.insert(
                "organId".to_string(),
                BmbpValue::from(format!("%/{}/%", path_id)),
            );
            query_script.filter(" organ_id_path like #{organId}");
        }
        let organ_list = BmbpORM
            .await
            .generate_script_query_list::<BmbpRbacOrgan>(
                &query_script.to_query_sql(),
                &script_param,
            )
            .await?;
        match organ_list {
            None => Ok(vec![]),
            Some(organ_vec) => {
                let organ_tree_list = TreeBuilder::build::<BmbpRbacOrgan>(organ_vec);
                Ok(organ_tree_list)
            }
        }
    }
    /// 分页查询组织列表
    pub async fn find_organ_page(
        params: &PageParams<OrganQueryParam>,
    ) -> BmbpResp<PageRespVo<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::raw_query_script();
        let mut path_id = "".to_string();

        match params.get_params() {
            None => {}
            Some(p) => {
                // 记录ID不为空
                if !p.get_r_id().is_empty() {
                    let organ = Self::find_organ_by_id(p.get_r_id()).await?;
                    if organ.is_some() {
                        path_id = organ.unwrap().get_organ_id().to_string();
                    }
                } else if !p.get_organ_id().is_empty() {
                    path_id = p.get_organ_id().to_string();
                } else if !p.get_organ_parent_id().is_empty() {
                    path_id = p.get_organ_parent_id().to_string();
                }
            }
        }

        if !path_id.is_empty() {
            script_param.insert(
                "organId".to_string(),
                BmbpValue::from(format!("%/{}/%", path_id)),
            );
            query_script.filter(" organ_id_path like #{organId}");
        }
        let organ_list = BmbpORM
            .await
            .generate_script_query_page::<BmbpRbacOrgan>(
                &query_script.to_query_sql(),
                &script_param,
                params.get_page_no().clone(),
                params.get_page_size().clone(),
            )
            .await?;
        Ok(organ_list)
    }
    /// 查询组织列表
    pub async fn find_organ_list(params: &OrganQueryParam) -> BmbpResp<Vec<BmbpRbacOrgan>> {
        let mut script_param = BmbpHashMap::new();
        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::raw_query_script();
        let mut path_id = "".to_string();
        // 记录ID不为空
        if !params.get_r_id().is_empty() {
            let organ = Self::find_organ_by_id(params.get_r_id()).await?;
            if organ.is_some() {
                path_id = organ.unwrap().get_organ_id().to_string();
            }
        } else if !params.get_organ_id().is_empty() {
            path_id = params.get_organ_id().to_string();
        } else if !params.get_organ_parent_id().is_empty() {
            path_id = params.get_organ_parent_id().to_string();
        }
        if !path_id.is_empty() {
            script_param.insert("organId".to_string(), BmbpValue::from(path_id));
            query_script.filter("ORGAN_ID_PATH '%/#{organId}/%'");
        }
        let organ_list = BmbpORM
            .await
            .generate_script_query_list::<BmbpRbacOrgan>(
                &query_script.to_query_sql(),
                &script_param,
            )
            .await?;
        match organ_list {
            None => Ok(vec![]),
            Some(organ_vec) => {
                let organ_tree_list = TreeBuilder::build::<BmbpRbacOrgan>(organ_vec);
                Ok(organ_tree_list)
            }
        }
    }
    /// 查询组织详情-> 多个参数
    pub async fn find_organ_one(organ: &BmbpRbacOrgan) -> BmbpResp<Option<BmbpRbacOrgan>> {
        if !organ.get_r_id().is_empty() {
            return Self::find_organ_by_id(organ.get_r_id()).await;
        }
        if !organ.get_organ_id().is_empty() {
            return Self::find_organ_by_organ_code(organ.get_organ_id()).await;
        }
        Ok(None)
    }
    /// 查询组织详情-通过R_ID
    pub async fn find_organ_by_id(r_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let mut script_sql: BmbpScriptSql = BmbpRbacOrgan::raw_query_script();
        let mut params = HashMap::new();
        params.insert("rId".to_string(), BmbpValue::from(r_id));
        script_sql.filter("r_id = #{rId}");
        let rs = BmbpORM
            .await
            .generate_script_query_one::<BmbpRbacOrgan>(&script_sql.to_query_sql(), &params)
            .await?;
        Ok(rs)
    }
    /// 查询组织详情-通过ORGAN-CODE
    pub async fn find_organ_by_organ_code(organ_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        let mut script_sql: BmbpScriptSql = BmbpRbacOrgan::raw_query_script();
        let mut params = HashMap::new();
        params.insert("organId".to_string(), BmbpValue::from(organ_id));
        script_sql.filter("organ_id = #{organId}");

        let rs = BmbpORM
            .await
            .generate_script_query_one::<BmbpRbacOrgan>(&script_sql.to_query_sql(), &params)
            .await?;
        Ok(rs)
    }
    /// 保存组织
    pub async fn save_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        if organ.get_r_id().is_empty() && organ.get_organ_id().is_empty() {
            Self::insert_organ(organ).await
        } else {
            Self::update_organ(organ).await
        }
    }
    /// 新增组织
    pub async fn insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Self::valid_insert_organ(organ)?;

        insert_decorate(organ);

        organ.set_r_id(uuid_upper());
        organ.set_organ_id(uuid_upper());

        if organ.get_organ_data_id().is_empty() {
            organ.set_organ_data_id(uuid_upper());
        } else {
            Self::check_same_data_id(organ).await?;
        }

        if organ.get_organ_parent_id().is_empty() {
            organ.set_organ_parent_id(ROOT_TREE_NODE.to_string());
            organ.set_organ_id_path(format!("/{}/", organ.get_organ_id()));
            organ.set_organ_title_path(format!("/{}/", organ.get_organ_title()));
        } else {
            let parent_organ_op =
                Self::find_organ_by_organ_code(organ.get_organ_parent_id()).await?;
            match parent_organ_op {
                None => {
                    return Err(BmbpError::api("所指定的父级节点不存在".to_string()));
                }
                Some(p_organ) => {
                    organ.set_organ_id_path(format!(
                        "{}{}/",
                        p_organ.get_organ_id_path(),
                        organ.get_organ_id()
                    ));
                    organ.set_organ_title_path(format!(
                        "{}{}/",
                        p_organ.get_organ_title_path(),
                        organ.get_organ_title()
                    ));
                }
            }
        }
        Self::check_same_organ_title(organ).await?;
        let insert_sql: BmbpScriptSql = BmbpRbacOrgan::insert_script();
        let params = BmbpValue::from(organ.clone()).raw_map().unwrap().clone();
        tracing::debug!(
            "新增组织数据：{:#?}",
            serde_json::to_string_pretty(&params).unwrap()
        );
        let rs = BmbpORM
            .await
            .script_insert(&insert_sql.to_insert_sql(), &params)
            .await?;
        Ok(rs)
    }
    /// 更新组织
    pub async fn update_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        let old_organ_rs = Self::find_organ_one(organ).await?;
        if old_organ_rs.is_none() {
            return Err(BmbpError::api("未查询到有效的组织，无法更新".to_string()));
        }
        let old_organ = old_organ_rs.unwrap();
        if old_organ.get_organ_title().is_empty() {}
        Ok(0)
    }

    /// 更新组织状态
    pub async fn update_organ_status(id: String, status: String) -> BmbpResp<usize> {
        Ok(0)
    }
    /// 更新组织上级
    pub async fn update_organ_parent(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Ok(0)
    }
    /// 删除组织
    pub async fn remove_organ_by_id(id: String) -> BmbpResp<usize> {
        Ok(0)
    }
}

/// 校验逻辑
impl OrganService {
    /// 保存时的数据校验
    pub fn valid_insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if organ.get_organ_title().is_empty() {
            return Err(BmbpError::valid("组织名称不允许为空"));
        }
        Ok(true)
    }
    /// 判断是否包含相同的数据关联
    pub async fn check_same_data_id(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
    /// 判断是否包含相同组织
    pub async fn check_same_organ_title(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        let mut params = HashMap::new();
        params.insert(
            "organTitle".to_string(),
            BmbpValue::from(organ.get_organ_title()),
        );
        params.insert(
            "organParentId".to_string(),
            BmbpValue::from(organ.get_organ_parent_id()),
        );

        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::query_script();
        query_script
            .filter("organ_title = #{organTitle}")
            .filter("organ_parent_id = #{organParentId}");

        let organ_vec_op = BmbpORM
            .await
            .script_query_list(&query_script.to_query_sql(), &params)
            .await?;
        match organ_vec_op {
            None => Ok(true),
            Some(organ_vec) => {
                if organ_vec.is_empty() {
                    Ok(true)
                } else {
                    Err(BmbpError::api(
                        "已存在相同名称的组织，请重新添加".to_string(),
                    ))
                }
            }
        }
    }
    /// 判断是否包含下级
    pub async fn check_organ_has_children(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        let mut params = HashMap::new();
        params.insert(
            "organTitle".to_string(),
            BmbpValue::from(organ.get_organ_title()),
        );
        params.insert(
            "organParentId".to_string(),
            BmbpValue::from(organ.get_organ_parent_id()),
        );

        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::query_script();
        query_script
            .filter("organ_title = #{organTitle}")
            .filter("organ_parent_id = #{organParentId}");

        let organ_vec_op = BmbpORM
            .await
            .script_query_list(&query_script.to_query_sql(), &params)
            .await?;
        match organ_vec_op {
            None => Ok(true),
            Some(organ_vec) => {
                if organ_vec.is_empty() {
                    Ok(true)
                } else {
                    Err(BmbpError::api(
                        "已存在相同名称的组织，请重新添加".to_string(),
                    ))
                }
            }
        }
    }
    /// 判断是否关联用户
    pub async fn check_organ_has_user(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        let mut params = HashMap::new();
        params.insert(
            "organTitle".to_string(),
            BmbpValue::from(organ.get_organ_title()),
        );
        params.insert(
            "organParentId".to_string(),
            BmbpValue::from(organ.get_organ_parent_id()),
        );

        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::query_script();
        query_script
            .filter("organ_title = #{organTitle}")
            .filter("organ_parent_id = #{organParentId}");

        let organ_vec_op = BmbpORM
            .await
            .script_query_list(&query_script.to_query_sql(), &params)
            .await?;
        match organ_vec_op {
            None => Ok(true),
            Some(organ_vec) => {
                if organ_vec.is_empty() {
                    Ok(true)
                } else {
                    Err(BmbpError::api(
                        "已存在相同名称的组织，请重新添加".to_string(),
                    ))
                }
            }
        }
    }
    /// 判断是否关联业务
    #[allow(dead_code)]
    pub async fn check_organ_has_data(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        let mut params = HashMap::new();
        params.insert(
            "organTitle".to_string(),
            BmbpValue::from(organ.get_organ_title()),
        );
        params.insert(
            "organParentId".to_string(),
            BmbpValue::from(organ.get_organ_parent_id()),
        );

        let mut query_script: BmbpScriptSql = BmbpRbacOrgan::query_script();
        query_script
            .filter("organ_title = #{organTitle}")
            .filter("organ_parent_id = #{organParentId}");

        let organ_vec_op = BmbpORM
            .await
            .script_query_list(&query_script.to_query_sql(), &params)
            .await?;
        match organ_vec_op {
            None => Ok(true),
            Some(organ_vec) => {
                if organ_vec.is_empty() {
                    Ok(true)
                } else {
                    Err(BmbpError::api(
                        "已存在相同名称的组织，请重新添加".to_string(),
                    ))
                }
            }
        }
    }
}
