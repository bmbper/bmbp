use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo};
use bmbp_app_utils::is_empty_string;
use bmbp_rdbc_orm::{Delete, Insert, RDBC_DATA_STATUS, RDBC_DISABLE, RDBC_ENABLE, RdbcModel, RdbcORM, RdbcPage, Update};
use crate::dict::model::{BmbpSettingDict, BmbpSettingDictOrmModel, DictQueryParams};
use crate::dict::scripts::build_query_script;

pub async fn query_dict_tree(params: DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
    if let Some(dict_list) = query_dict_list(params).await? {
        //TODO 转换为树节构
        return Ok(Some(dict_list));
    }
    Ok(None)
}

pub async fn query_dict_list(params: DictQueryParams) -> BmbpResp<Option<Vec<BmbpSettingDictOrmModel>>> {
    let query = build_query_script();
    match RdbcORM.await.select_list_by_query::<BmbpSettingDictOrmModel>(&query).await {
        Ok(data) => Ok(data),
        Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
    }
}

pub async fn query_dict_page(params: BmbpPageParam<DictQueryParams>) -> BmbpResp<PageVo<BmbpSettingDictOrmModel>> {
    // 拼接查询条件
    let query = build_query_script();
    let mut page: RdbcPage<BmbpSettingDictOrmModel> = RdbcPage::new();
    page.set_page_num(params.get_page_no().clone()).set_page_size(params.get_page_size().clone());
    match RdbcORM.await.select_page_by_query::<BmbpSettingDictOrmModel>(&mut page, &query).await {
        Ok(page_) => {
            let rbac_page = PageVo::new_page(page_.page_num().clone(),
                                             page_.page_size().clone(), page_.total().clone(), page.data_take());
            Ok(rbac_page)
        }
        Err(e) => Err(BmbpError::service(e.get_msg().as_str()))
    }
}


pub async fn query_dict_by_id(id: Option<String>) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
    if is_empty_string(id.as_ref()) {
        return Ok(None);
    }
    let mut query = build_query_script();
    query.eq(BmbpSettingDict::get_table_primary_key(), id.unwrap());
    match RdbcORM.await.select_one_by_query::<BmbpSettingDictOrmModel>(&query).await {
        Ok(data) => Ok(data),
        Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
    }
}

pub async fn insert_dict_info(mut dict: BmbpSettingDictOrmModel) -> BmbpResp<usize> {
    let mut insert = Insert::new();
    insert.insert_table(BmbpSettingDict::get_table_name());
    match RdbcORM.await.execute_insert(&insert).await {
        Ok(data) => {
            Ok(data as usize)
        }
        Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
    }
}

pub async fn update_dict_info(dict: BmbpSettingDictOrmModel) -> BmbpResp<usize> {
    let mut update = Update::new();
    update.update_table(BmbpSettingDict::get_table_name()).set(RDBC_DATA_STATUS, RDBC_DISABLE).eq(BmbpSettingDict::get_table_primary_key(), dict.get_data_id().unwrap());
    execute_update(&update).await
}


pub async fn disable_dict_status(dict_id: Option<String>) -> BmbpResp<usize> {
    if is_empty_string(dict_id.as_ref()) {
        return Err(BmbpError::service("请指定待停用的字典!"));
    }
    let mut update = Update::new();
    update.update_table(BmbpSettingDict::get_table_name()).set(RDBC_DATA_STATUS, RDBC_ENABLE).eq(BmbpSettingDict::get_table_primary_key(), dict_id.unwrap());
    execute_update(&update).await
}

pub async fn enable_dict_status(dict_id: Option<String>) -> BmbpResp<usize> {
    if is_empty_string(dict_id.as_ref()) {
        return Err(BmbpError::service("请指定待启用的字典!"));
    }
    let mut update = Update::new();
    update.update_table(BmbpSettingDict::get_table_name()).set(RDBC_DATA_STATUS, RDBC_DISABLE).eq(BmbpSettingDict::get_table_primary_key(), dict_id.unwrap());
    execute_update(&update).await
}

async fn execute_update(update:&Update)->BmbpResp<usize>{
    match RdbcORM.await.execute_update(update).await {
        Ok(data) => {
            Ok(data as usize)
        }
        Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
    }
}

pub async fn delete_dict(dict_id: Option<String>) -> BmbpResp<usize> {
    if is_empty_string(dict_id.as_ref()) {
        return Err(BmbpError::service("请指定待删除的字典!"));
    }
    let mut delete_dict = Delete::new();
    delete_dict.delete_table(BmbpSettingDict::get_table_name()).eq(BmbpSettingDict::get_table_primary_key(), dict_id.unwrap());
    match RdbcORM.await.execute_delete(&delete_dict).await {
        Ok(data) => Ok((data as usize)),
        Err(err) => Err(BmbpError::service(err.get_msg().as_str()))
    }
}