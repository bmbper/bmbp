use serde::{Deserialize, Serialize};

use bmbp_app_common::{BmbpHashMap, BmbpValue};
use bmbp_app_common::BmbpBaseModelTrait;

use crate::{date_time_now, is_empty_prop, simple_uuid_upper};
use crate::crypto::md5_encode;

pub fn insert_decorate<'a, T>(data: &mut T)
where
    T: BmbpBaseModelTrait + Deserialize<'a> + Serialize,
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
    T: BmbpBaseModelTrait + Deserialize<'a> + Serialize,
{
    let now_time = date_time_now();
    data.set_base_r_update_time(now_time);
    data.set_base_r_create_user("0".to_string());
}

fn sign_data<'a, T>(data: &mut T)
where
    T: BmbpBaseModelTrait + Deserialize<'a> + Serialize,
{
    data.set_base_r_sign("".to_string());
    let data_json_string = serde_json::to_string(data).unwrap();
    let sign = md5_encode(data_json_string);
    data.set_base_r_sign(sign);
}

#[allow(dead_code)]
pub fn valid_sign<'a, T>(data: &mut T) -> bool
where
    T: BmbpBaseModelTrait + Deserialize<'a> + Serialize,
{
    let old_sign = data.get_base_r_sign().clone();
    sign_data(data);
    let is_valid = old_sign.eq(data.get_base_r_sign());
    data.set_base_r_sign(old_sign);
    is_valid
}

pub fn add_insert_default_value(params: &mut BmbpHashMap) {
    if is_empty_prop(params, "dataId") {
        params.insert("dataId".to_string(), BmbpValue::from(simple_uuid_upper()));
    }

    if is_empty_prop(params, "recordLevel") {
        params.insert("recordLevel".to_string(), BmbpValue::from("0"));
    }

    if is_empty_prop(params, "recordStatus") {
        params.insert("recordStatus".to_string(), BmbpValue::from("0"));
    }

    if is_empty_prop(params, "recordFlag") {
        params.insert("recordFlag".to_string(), BmbpValue::from("0"));
    }

    params.insert("recordCreateUser".to_string(), BmbpValue::from("0"));
    params.insert(
        "recordCreateTime".to_string(),
        BmbpValue::from(date_time_now()),
    );
    params.insert("recordUpdateUser".to_string(), BmbpValue::from("0"));
    params.insert(
        "recordUpdateTime".to_string(),
        BmbpValue::from(date_time_now()),
    );

    if is_empty_prop(params, "recordOwnerOrg") {
        params.insert("recordOwnerOrg".to_string(), BmbpValue::from("_"));
    }
    if is_empty_prop(params, "recordOwnerUser") {
        params.insert("recordOwnerUser".to_string(), BmbpValue::from("_"));
    }
    if is_empty_prop(params, "recordSign") {
        params.insert("recordSign".to_string(), BmbpValue::from("_"));
    }
    if is_empty_prop(params, "recordNum") {
        params.insert("recordNum".to_string(), BmbpValue::from(0));
    }
    if is_empty_prop(params, "recordRemark") {
        params.insert("recordRemark".to_string(), BmbpValue::from(""));
    }
}

pub fn add_update_default_value(params: &mut BmbpHashMap) {
    params.insert("recordUpdateUser".to_string(), BmbpValue::from("0"));
    params.insert(
        "recordUpdateTime".to_string(),
        BmbpValue::from(date_time_now()),
    );
}
