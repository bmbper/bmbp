pub use model::*;
pub use route::*;
mod dao;
mod model;
mod route;
mod service;
mod util;
mod valid;

pub use route::change_organ_parent;

#[cfg(test)]
mod tests {
    use bmbp_types::vo::BaseOrmModel;
    use bmbp_types::BmbpResp;
    use bmbp_util::uuid_upper;

    use crate::organ::model::{BmbpOrganType, BmbpRbacOrgan};
    use crate::organ::service::OrganService;

    #[tokio::test]
    async fn save_organ() {
        tracing_subscriber::fmt().init();
        let mut params = BmbpRbacOrgan::new();
        params.set_organ_title("中国简竹集团".to_string());
        params.set_organ_type(BmbpOrganType::Unit);
        params.set_organ_data_id(uuid_upper());
        let organ_rs = OrganService::save_organ(&mut params).await;
        match organ_rs {
            Ok(organ) => {
                println!("organ:{:#?}", organ);
                assert!(!organ.get_r_id().is_empty(), "{}", true)
            }
            Err(e) => {
                println!("err:{:#?}", e);
                assert!(e.to_string().is_empty(), "{}", true)
            }
        }
    }
}
