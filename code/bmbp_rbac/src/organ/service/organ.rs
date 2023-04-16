use bmbp_types::vo::BaseOrmModel;
use bmbp_types::{BmbpError, BmbpMap, BmbpResp, BmbpValue, PageInner, ROOT_TREE_NODE};
use bmbp_util::{simple_uuid_upper, TreeBuilder};

use crate::organ::dao::OrganDao;
use crate::organ::model::{BmbpRbacOrgan, OrganQueryParam};
use crate::util::{append_create_vo, append_update_vo};

pub struct OrganService();

impl OrganService {
    /// 查询组织树
    pub(crate) async fn find_organ_tree(params: &OrganQueryParam) -> BmbpResp<Vec<BmbpRbacOrgan>> {
        tracing::info!("组织机构服务-调用组织树查询");
        let rows = Self::find_organ_list(params).await?;
        match rows {
            None => Ok(vec![]),
            Some(v) => {
                let tree_organ = TreeBuilder::build(v);
                Ok(tree_organ)
            }
        }
    }
    /// 查询分页
    pub(crate) async fn find_organ_page(
        params: &mut OrganQueryParam,
    ) -> BmbpResp<PageInner<BmbpRbacOrgan>> {
        tracing::info!("组织机构服务-调用组织分页查询");
        let page_inner = OrganDao::find_organ_page(params).await?;
        Ok(page_inner)
    }
    /// 查询组织列表
    pub async fn find_organ_list(params: &OrganQueryParam) -> BmbpResp<Option<Vec<BmbpRbacOrgan>>> {
        OrganDao::find_organ_list(params).await
    }
    /// 查询详情
    pub async fn find_organ_info_by_rid(r_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        tracing::info!("调用组织服务-查询组织详情-根据当前节点");
        let organ: Option<BmbpRbacOrgan> = OrganDao::find_organ_info_by_rid(r_id).await?;
        Ok(organ)
    }
    /// 查询详情
    pub async fn find_organ_info_by_organ_id(organ_id: &String) -> BmbpResp<Option<BmbpRbacOrgan>> {
        tracing::info!("调用组织服务-查询组织详情-根据当前节点");
        let organ: Option<BmbpRbacOrgan> = OrganDao::find_organ_info_by_organ_id(organ_id).await?;
        Ok(organ)
    }
    /// 保存组织
    pub(crate) async fn save_organ(params: &mut BmbpRbacOrgan) -> BmbpResp<BmbpRbacOrgan> {
        Ok(params.clone())
    }
    /// 新增组织
    pub(crate) async fn insert_organ(params: &mut BmbpRbacOrgan) -> BmbpResp<()> {
        let row_count = OrganDao::insert_organ(params).await?;
        if row_count != 1 {
            return Err(BmbpError::api("新增组织机构失败，记录为0条".to_string()));
        }
        Ok(())
    }
    /// 更新组织
    pub(crate) async fn update_organ(params: &mut BmbpRbacOrgan) -> BmbpResp<()> {
        Ok(())
    }
    /// 修改组织上级
    pub(crate) async fn change_organ_parent(param: &OrganQueryParam) -> BmbpResp<usize> {
        // 判断传入的上级参数是否为空
        if param.get_parent_organ_id().is_empty() {
            return Err(BmbpError::api(
                "更改组织上级时，组织上级ID不允许为空".to_string(),
            ));
        }

        // 查询当前节点
        let mut current_organ_params = OrganQueryParam::default();
        current_organ_params.set_r_id(param.get_r_id().to_string());
        let current_organ_node = Self::find_organ_info_by_rid(param.get_r_id()).await?;
        if current_organ_node.is_none() {
            return Err(BmbpError::api("指定的组织不存在，无法修改上级".to_string()));
        }
        let current_organ = current_organ_node.unwrap();

        // 获取父级节点
        let parent_node;
        if !param.get_parent_organ_id().eq(&ROOT_TREE_NODE.clone()) {
            let mut parent_organ_params = OrganQueryParam::new();
            parent_organ_params.set_organ_id(param.get_parent_organ_id().clone());
            if let Some(p_vo) = Self::find_organ_info_by_rid(param.get_r_id()).await? {
                parent_node = p_vo;
            } else {
                return Err(BmbpError::api("更改组织上级时，组织上级不存在".to_string()));
            }
        } else {
            // 根节点
            let mut parent_vo = BmbpRbacOrgan::new();
            parent_vo.set_organ_id("0".to_string());
            parent_vo.set_organ_path("/".to_string());
            parent_node = parent_vo;
        }

        let target_path = format!(
            "{}{}/",
            parent_node.get_organ_path(),
            current_organ.get_organ_title()
        );
        let current_path = current_organ.get_organ_path();
        let mut update_params = BmbpMap::new();
        update_params.insert("targetPath".to_string(), BmbpValue::String(target_path));
        update_params.insert(
            "currentPath".to_string(),
            BmbpValue::String(current_path.to_string()),
        );
        OrganDao::update_organ_parent(&update_params).await
    }
    /// 删除组织
    pub(crate) async fn delete_organ(r_id: &String) -> BmbpResp<usize> {
        if r_id.is_empty() {
            return Err(BmbpError::api("删除组织时，请指定删除凭证".to_string()));
        }
        OrganDao::delete_organ(r_id).await
    }
}
