use async_trait::async_trait;
use bmbp_orm_ins::BmbpOrmSQL;
use bmbp_types::{BmbpResp, PageInner};
use serde::Serialize;

pub trait UpdatePropParam: Clone + Send {
    fn get_id(&self) -> &String;
    fn get_r_id(&self) -> &String;
    fn get_prop(&self) -> &String;
}

#[async_trait]
pub trait UpdatePropService {
    #[allow(unused)]
    async fn update_prop(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}

#[async_trait]
pub trait UpdateLevelService {
    #[allow(unused)]
    async fn update_level(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}
#[async_trait]
pub trait UpdateFlagService {
    #[allow(unused)]
    async fn update_flag(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}
#[async_trait]
pub trait UpdateStatusService {
    #[allow(unused)]
    async fn update_status(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}
#[async_trait]
pub trait UpdateSignService {
    #[allow(unused)]
    async fn update_sign(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}

#[async_trait]
pub trait UpdateOwnerOrgService {
    #[allow(unused)]
    async fn update_owner_org(&self, params: impl UpdatePropParam) -> BmbpResp<usize> {
        Ok(0)
    }
}

pub trait UpdateBasePropParam {
    fn get_id(&self) -> &String;
    fn get_r_id(&self) -> &String;
    fn get_level(&self) -> &String;
    fn get_flag(&self) -> &String;
    fn get_sign(&self) -> &String;
    fn get_status(&self) -> &String;
    fn get_owner_org(&self) -> &String;
}

pub trait UpdateBasePropsService:
    UpdateFlagService
    + UpdateOwnerOrgService
    + UpdateSignService
    + UpdateStatusService
    + UpdateLevelService
{
}

#[async_trait]
pub trait CurdService {
    async fn find_list<Q, V>(&self, params: &Q) -> BmbpResp<Vec<V>>;
    async fn find_one<Q, V>(&self, params: &Q) -> BmbpResp<Option<V>>;
    async fn delete<Q, V>(&self, params: &Q) -> BmbpResp<usize>;
    async fn insert<V>(&self, po: &V) -> BmbpResp<usize>;
    async fn update<Q, V>(&self, params: &Q, po: &V) -> BmbpResp<usize>;
    async fn save<Q, V>(&self, params: &Q, po: &V) -> BmbpResp<usize>;
}

#[async_trait]
pub trait CurdPageService: CurdService {
    async fn find_page<Q, V>(&self, params: &Q) -> BmbpResp<PageInner<V>>
    where
        V: Clone + Default + Serialize + Send + Sync;
}

#[async_trait]
pub trait CurdTreeService: CurdService {
    async fn find_tree<Q, V>(&self, params: &Q) -> BmbpResp<PageInner<V>>
    where
        V: Clone + Default + Serialize + Send + Sync;
}

#[async_trait]
pub trait CurdTreePageService: CurdPageService + CurdTreeService {}

#[async_trait]
pub trait CurdDao {
    fn orm_query_sql<Q>(&self, params: &Q) -> BmbpResp<BmbpOrmSQL>;
    fn orm_delete_sql<Q>(&self, params: &Q) -> BmbpResp<BmbpOrmSQL>;
    fn orm_insert_sql<V>(&self, po: &V) -> BmbpResp<BmbpOrmSQL>;
    fn orm_update_sql<Q, V>(&self, params: &Q, po: &V) -> BmbpResp<BmbpOrmSQL>;
    async fn find_page<Q, V>(
        &self,
        params: Q,
        page_no: usize,
        page_size: usize,
    ) -> BmbpResp<PageInner<V>>
    where
        V: Clone + Default + Serialize + Send + Sync;
    async fn find_list<Q, V>(&self, params: &Q) -> BmbpResp<Vec<V>>;
    async fn find_one<Q, V>(&self, params: &Q) -> BmbpResp<Option<V>>;
    async fn delete<Q>(&self, params: &Q) -> BmbpResp<usize>;
    async fn insert<V>(&self, po: &V) -> BmbpResp<usize>;
    async fn update<Q, V>(&self, params: &Q, po: &V) -> BmbpResp<usize>;
}
