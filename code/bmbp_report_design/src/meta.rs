use bmbp_app_common::BmbpResp;

pub trait ModelParser {}

pub trait ModelToSQL {
    fn to_sql(&self) -> BmbpResp<String>;
}
