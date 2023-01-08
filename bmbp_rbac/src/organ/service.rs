use bmbp_types::vo::{BaseOrmVoPo, BmbpApiQueryParam};
use bmbp_types::{BmbpError, BmbpResp, ROOT_TREE_NODE};
use bmbp_util::{simple_uuid_upper, uuid_upper, TreeBuilder};

use crate::organ::dao::OrganDao;
use crate::organ::save_organ;
use crate::organ::vopo::{
    BmbpOrganDeptVo, BmbpOrganPersonVo, BmbpOrganPostVo, BmbpOrganUnitVo, BmbpOrganUnitsVo,
};
use crate::util::{append_create_vo, append_update_vo};

use super::vopo::{BmbpOrganVo, QueryParam};

pub struct OrganService();

impl OrganService {
    pub async fn find_tree_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let rows = Self::find_grid_data(params).await?;
        let tree_organ = TreeBuilder::build(rows);
        Ok(tree_organ)
    }

    pub async fn find_grid_data(params: &QueryParam) -> BmbpResp<Vec<BmbpOrganVo>> {
        let rows = OrganDao::find_grid_data(params).await?;
        Ok(rows)
    }

    pub async fn find_organ_info_by_organ_id(params: &QueryParam) -> BmbpResp<Option<BmbpOrganVo>> {
        let organ: Option<BmbpOrganVo> = OrganDao::find_one_by_organ_id(params).await?;
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

        if params.get_parent_organ_id().eq(&ROOT_TREE_NODE.to_string()) {
            params.set_organ_path("/".to_string());
        } else {
            let mut query_params = QueryParam::default();
            query_params.set_organ_id(params.get_parent_organ_id().clone());
            if let Some(parent_node) =
                OrganService::find_organ_info_by_organ_id(&mut query_params).await?
            {
                let new_path =
                    parent_node.get_organ_path().to_string() + params.get_organ_title().as_str();
                params.set_organ_path(new_path);
            } else {
                let new_path = "/".to_string() + params.get_organ_title().as_str();
                params.set_organ_path(new_path);
            }
        }
        Ok(())
    }
}
