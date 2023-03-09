use bmbp_types::vo::BaseOrmModel;
use bmbp_util::crypto::md5_encode;
use bmbp_util::{date_time_now, uuid};
use serde::Serialize;
pub struct VoUtil;

#[allow(dead_code)]
impl VoUtil {
    pub fn is_new<T>(vo: &T) -> bool
    where
        T: BaseOrmModel + Serialize,
    {
        vo.get_r_id().is_empty()
    }
    pub fn append_insert<T>(vo: &mut T)
    where
        T: BaseOrmModel + Serialize,
    {
        let current_user = "_".to_string();
        let current_time = date_time_now();
        vo.set_r_create_user(current_user.clone());
        vo.set_r_update_user(current_user.clone());
        vo.set_r_create_time(current_time.clone());
        vo.set_r_update_time(current_time.clone());
        vo.set_r_owner_org("_".to_string());

        if vo.get_r_id().is_empty() {
            vo.set_r_id(uuid());
        }

        if vo.get_r_level().is_empty() {
            vo.set_r_level("0".to_string());
        }
        if vo.get_r_flag().is_empty() {
            vo.set_r_flag("0".to_string());
        }
        vo.set_r_sign("".to_string());
    }
    pub fn append_update<T>(vo: &mut T)
    where
        T: BaseOrmModel + Serialize,
    {
        let current_user = "_".to_string();
        let current_time = date_time_now();
        vo.set_r_update_user(current_user.clone());
        vo.set_r_update_time(current_time.clone());
    }

    pub fn append_sign<T>(vo: &mut T)
    where
        T: BaseOrmModel + Serialize,
    {
        let sign = Self::get_sign::<T>(vo);
        vo.set_r_sign(sign);
    }

    pub fn get_sign<T>(vo: &mut T) -> String
    where
        T: BaseOrmModel + Serialize,
    {
        let old_sign = vo.get_r_sign().clone();
        vo.set_r_sign("".to_string());
        let sign_string = serde_json::to_string(vo).unwrap();
        let encode_sign = md5_encode(sign_string);
        vo.set_r_sign(old_sign);
        encode_sign
    }

    pub fn valid_sign<T>(vo: &mut T) -> bool
    where
        T: BaseOrmModel + Serialize,
    {
        if vo.get_r_sign().is_empty() {
            return false;
        }

        let old_sign = vo.get_r_sign().clone();
        vo.set_r_sign("".to_string());

        let sign_string = serde_json::to_string(vo).unwrap();
        let encode_sign = md5_encode(sign_string);

        old_sign.eq(encode_sign.as_str())
    }
}
