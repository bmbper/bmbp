use bmbp_app_common::{BmbpError, BmbpPageParam, BmbpResp, PageVo, RespVo};
use bmbp_rdbc_orm::{RdbcORM, RdbcPage};
use crate::dict::model::{BmbpSettingDictOrmModel, DictQueryParams};
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
    Ok(None)
}

pub async fn insert_dict_info(dict: BmbpSettingDictOrmModel) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
    Ok(None)
}

pub async fn update_dict_info(dict: BmbpSettingDictOrmModel) -> BmbpResp<Option<BmbpSettingDictOrmModel>> {
    Ok(None)
}