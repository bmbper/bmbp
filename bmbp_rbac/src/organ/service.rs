use bmbp_types::vo::{BaseOrmVoPo, BmbpApiQueryParam};
use bmbp_types::{BmbpError, BmbpResp, PageInner, ROOT_TREE_NODE};
use bmbp_util::{simple_uuid_upper, uuid_upper, TreeBuilder};

use crate::organ::dao::OrganDao;
use crate::organ::save_organ;
use crate::organ::vopo::{
    BmbpOrganDeptVo, BmbpOrganPersonVo, BmbpOrganPostVo, BmbpOrganUnitVo, BmbpOrganUnitsVo,
    PageQueryParam,
};
use crate::util::{append_create_vo, append_update_vo};

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganService();

impl OrganService {
    pub(crate) async fn find_organ_tree(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let rows = Self::find_organ_list(params).await?;
        let tree_organ = TreeBuilder::build(rows);
        Ok(tree_organ)
    }

    pub(crate) async fn find_organ_tree_by_parent(
        query_params: &mut QueryParam,
    ) -> BmbpResp<Vec<BmbpOrganVo>> {
        tracing::info!("调用组织服务-查询组织树-根据上级节点");
        if query_params.get_parent_organ_id().is_empty() {
            return Ok(vec![]);
        }
        let mut new_query_params = QueryParam::default();
        new_query_params.set_organ_id(query_params.get_parent_organ_id().clone());
        Self::find_organ_tree_by_organ_id(&mut new_query_params).await
    }

    pub(crate) async fn find_organ_tree_by_organ_id(
        mut query_params: &mut QueryParam,
    ) -> BmbpResp<Vec<BmbpOrganVo>> {
        tracing::info!("调用组织服务-查询组织树-根据当前组织ID");
        if query_params.get_organ_id().is_empty() {
            return Ok(vec![]);
        }

        if let Some(parent_node) = OrganService::find_organ_info_by_organ_id(query_params).await? {
            let mut new_query_params = QueryParam::default();
            new_query_params.set_organ_path(parent_node.get_organ_path().clone());
            Self::find_organ_tree_by_organ_path(&mut new_query_params).await
        } else {
            return Ok(vec![]);
        }
    }

    pub(crate) async fn find_organ_tree_by_organ_path(
        params: &QueryParam,
    ) -> BmbpResp<Vec<BmbpOrganVo>> {
        if params.get_organ_path().is_empty() {
            return Ok(vec![]);
        }
        let rows = Self::find_organ_list(params).await?;
        let tree_organ = TreeBuilder::build(rows);
        Ok(tree_organ)
    }
}

impl OrganService {
    pub(crate) async fn find_organ_page(
        params: &mut PageQueryParam,
    ) -> BmbpResp<PageInner<BmbpOrganVo>> {
        tracing::info!("组织机构服务-调用分页查询");
        let page_inner = OrganDao::find_organ_page(params).await?;
        Ok(page_inner)
    }

    pub async fn find_organ_list(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        OrganDao::find_organ_list(params).await
    }

    pub async fn find_organ_info_by_organ_id(params: &QueryParam) -> BmbpResp<Option<BmbpOrganVo>> {
        tracing::info!("调用组织服务-查询组织详情-根据当前节点");
        let organ: Option<BmbpOrganVo> = OrganDao::find_organ_info(params).await?;
        Ok(organ)
    }

    pub async fn save_organ(params: &mut BmbpOrganVo) -> BmbpResp<BmbpOrganVo> {
        Self::append_organ_vo(params).await?;
        if params.get_r_id().is_empty() {
            append_create_vo::<BmbpOrganVo>(params);
            Self::insert_organ(params).await?;
        } else {
            append_update_vo::<BmbpOrganVo>(params);
            Self::update_organ(params).await?;
        }
        Ok(params.clone())
    }

    pub async fn insert_organ(params: &mut BmbpOrganVo) -> BmbpResp<()> {
        let row_count = OrganDao::insert_organ(params).await?;
        if row_count != 1 {
            return Err(BmbpError::api_service(
                "新增组织机构失败，记录为0条".to_string(),
            ));
        }
        Ok(())
    }
    pub async fn update_organ(params: &mut BmbpOrganVo) -> BmbpResp<()> {
        OrganDao::update_organ(params).await?;
        Ok(())
    }

    pub async fn save_organ_units(params: &mut BmbpOrganUnitsVo) -> BmbpResp<BmbpOrganUnitsVo> {
        Ok(params.clone())
    }
    pub async fn save_organ_unit(params: &mut BmbpOrganUnitVo) -> BmbpResp<BmbpOrganUnitVo> {
        Ok(params.clone())
    }
    pub async fn save_organ_dept(params: &mut BmbpOrganDeptVo) -> BmbpResp<BmbpOrganDeptVo> {
        Ok(params.clone())
    }
    pub async fn save_organ_post(params: &mut BmbpOrganPostVo) -> BmbpResp<BmbpOrganPostVo> {
        Ok(params.clone())
    }

    pub async fn save_organ_person(params: &mut BmbpOrganPersonVo) -> BmbpResp<BmbpOrganPersonVo> {
        Ok(params.clone())
    }

    pub async fn append_organ_vo(params: &mut BmbpOrganVo) -> BmbpResp<()> {
        if params.get_organ_title().is_empty() {
            return Err(BmbpError::api_service("组织机构名称不能为空！".to_string()));
        }

        if params.get_organ_id().is_empty() {
            params.set_organ_id(simple_uuid_upper());
        }

        if params.get_parent_organ_id().is_empty() {
            params.set_parent_organ_id(ROOT_TREE_NODE.to_string());
        }

        if params.get_organ_data_id().is_empty() {
            params.set_organ_data_id(simple_uuid_upper());
        }

        let mut query_params = QueryParam::default();
        query_params.set_organ_id(params.get_parent_organ_id().clone());
        if let Some(parent_node) =
            OrganService::find_organ_info_by_organ_id(&mut query_params).await?
        {
            let new_path =
                parent_node.get_organ_path().to_string() + "/" + params.get_organ_title().as_str();
            params.set_organ_path(new_path);
        } else {
            let new_path = "/".to_string() + params.get_organ_title().as_str();
            params.set_organ_path(new_path);
            params.set_parent_organ_id(ROOT_TREE_NODE.to_string());
        }
        Ok(())
    }
}
