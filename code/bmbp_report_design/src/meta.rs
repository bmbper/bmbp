use bmbp_types::BmbpResp;

pub trait ModelParser {}

pub trait ModelToSQL {
    fn to_sql(&self) -> BmbpResp<String>;
}
