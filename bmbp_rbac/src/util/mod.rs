use bmbp_types::vo::BaseOrmVoPo;
use bmbp_util::{date_time_now, simple_uuid_upper};

pub fn append_create_vo<T>(vo: &mut T)
where
    T: BaseOrmVoPo,
{
    let now_time = date_time_now();
    vo.set_r_id(simple_uuid_upper())
        .set_r_flag("0".to_string())
        .set_r_level("0".to_string())
        .set_r_owner_org("0".to_string())
        .set_r_owner_user("0".to_string())
        .set_r_create_time(now_time.clone())
        .set_r_create_user("TODO".to_string())
        .set_r_update_user("TODO".to_string())
        .set_r_update_time(now_time.clone());
}

pub fn append_update_vo<T>(vo: &mut T)
where
    T: BaseOrmVoPo,
{
    let now_time = date_time_now();
    vo.set_r_update_user("TODO".to_string())
        .set_r_update_time(now_time.clone());
}
