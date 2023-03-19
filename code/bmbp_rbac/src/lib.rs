extern crate core;

pub use face::*;
pub use routes::build_rbac_router;

mod api;
mod app;
mod data;
mod face;
mod menu;
mod organ;
mod role;
mod routes;
mod user;
mod util;

#[cfg(test)]
mod tests {
    use bmbp_orm_ins::BmbpORM;
    use bmbp_types::BmbpResp;

    use crate::menu::BmbpMenuVo;
    use crate::organ::BmbpOrganModel;

    #[tokio::test]
    async fn test_rbac_list() -> BmbpResp<()> {
        let raw_query_sql = "SELECT * FROM A ".to_string();
        let organ_list: Vec<BmbpOrganModel> = BmbpORM
            .await
            .struct_query_list::<BmbpOrganModel>(&raw_query_sql)
            .await?;
        let organ_list: Vec<BmbpMenuVo> = BmbpORM
            .await
            .struct_query_list::<BmbpMenuVo>(&raw_query_sql)
            .await?;

        Ok(())
    }
}
