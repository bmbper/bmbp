use std::collections::HashMap;

use bmbp_orm_ins::{BmbpORM, BmbpScriptSql};
use bmbp_types::{BmbpError, BmbpHashMap, BmbpResp, BmbpValue, ROOT_TREE_NODE};
use bmbp_util::{insert_decorate, uuid_upper, TreeBuilder};

use crate::organ_model::{BmbpRbacOrgan, OrganQueryParam};

pub struct OrganService();

impl OrganService {
    pub async fn find_organ_tree(params: &OrganQueryParam) -> BmbpResp<Vec<BmbpRbacOrgan>> {
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
    pub async fn find_organ_by_organ_id(organ_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
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
    pub async fn save_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        if organ.get_r_id().is_empty() {
            Self::insert_organ(organ).await
        } else {
            Self::update_organ(organ).await
        }
    }
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
            let parent_organ_op = Self::find_organ_by_organ_id(organ.get_organ_parent_id()).await?;
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

    pub async fn update_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<usize> {
        Ok(0)
    }
}

impl OrganService {
    pub fn valid_insert_organ(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        if organ.get_organ_title().is_empty() {
            return Err(BmbpError::valid("组织名称不允许为空"));
        }
        Ok(true)
    }

    pub async fn check_same_data_id(organ: &mut BmbpRbacOrgan) -> BmbpResp<bool> {
        Ok(true)
    }
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
}
