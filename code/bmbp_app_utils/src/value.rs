use serde::{Deserialize, Serialize};

use bmbp_app_common::BmbpBaseModel;

use crate::crypto::md5_encode;
use crate::date_time_now;

pub fn insert_decorate<'a, T>(data: &mut T)
where
    T: BmbpBaseModel + Deserialize<'a> + Serialize,
{
    if data.get_base_r_level().is_empty() {
        data.set_base_r_level("0".to_string());
    }
    if data.get_base_r_flag().is_empty() {
        data.set_base_r_flag("0".to_string());
    }
    let now_time = date_time_now();
    data.set_base_r_create_time(now_time.clone());
    data.set_base_r_update_time(now_time);
    data.set_base_r_create_user("0".to_string());
    data.set_base_r_update_user("0".to_string());
    data.set_base_r_owner_user("0".to_string());
    data.set_base_r_owner_org("0".to_string());
    sign_data(data)
}

pub fn update_decorate<'a, T>(data: &mut T)
where
    T: BmbpBaseModel + Deserialize<'a> + Serialize,
{
    let now_time = date_time_now();
    data.set_base_r_update_time(now_time);
    data.set_base_r_create_user("0".to_string());
}

fn sign_data<'a, T>(data: &mut T)
where
    T: BmbpBaseModel + Deserialize<'a> + Serialize,
{
    data.set_base_r_sign("".to_string());
    let data_json_string = serde_json::to_string(data).unwrap();
    let sign = md5_encode(data_json_string);
    data.set_base_r_sign(sign);
}

#[allow(dead_code)]
pub fn valid_sign<'a, T>(data: &mut T) -> bool
where
    T: BmbpBaseModel + Deserialize<'a> + Serialize,
{
    let old_sign = data.get_base_r_sign().clone();
    sign_data(data);
    let is_valid = old_sign.eq(data.get_base_r_sign());
    data.set_base_r_sign(old_sign);
    is_valid
}
