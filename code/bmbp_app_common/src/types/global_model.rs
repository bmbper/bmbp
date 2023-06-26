use serde::Deserialize;
use serde::Serialize;

use crate::BmbpHashMap;
use crate::BmbpValue;
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BmbpBaseModel {
    /// 记录主键
    record_id: Option<String>,
    /// 记录密级
    record_level: Option<String>,
    /// 记录状态
    record_status: Option<String>,
    /// 记录标识
    record_flag: Option<String>,
    /// 记录显示顺序
    record_num: Option<usize>,
    /// 记录备注
    record_remark: Option<String>,
    /// 记录创建时间
    record_create_time: Option<String>,
    /// 记录创建人
    record_create_user: Option<String>,
    /// 记录更新时间
    record_update_time: Option<String>,
    /// 记录更新用户
    record_update_user: Option<String>,
    /// 记录所属用户
    record_owner_user: Option<String>,
    /// 记录所属组织
    record_owner_org: Option<String>,
    /// 记录防串改标识
    record_sign: Option<String>,
}

#[allow(dead_code)]
impl BmbpBaseModel {
    pub fn new() -> Self {
        BmbpBaseModel::default()
    }

    pub fn get_fields() -> Vec<String> {
        vec![
            "record_id".to_string(),
            "record_level".to_string(),
            "record_status".to_string(),
            "record_flag".to_string(),
            "record_num".to_string(),
            "record_remark".to_string(),
            "record_create_time".to_string(),
            "record_create_user".to_string(),
            "record_update_time".to_string(),
            "record_update_user".to_string(),
            "record_owner_org".to_string(),
            "record_owner_user".to_string(),
            "record_sign".to_string(),
        ]
    }

    pub fn set_record_id(&mut self, record_id: String) -> &mut Self {
        self.record_id = Some(record_id);
        self
    }
    pub fn set_record_level(&mut self, record_level: String) -> &mut Self {
        self.record_level = Some(record_level);
        self
    }
    pub fn set_record_status(&mut self, record_status: String) -> &mut Self {
        self.record_status = Some(record_status);
        self
    }

    pub fn set_record_flag(&mut self, record_flag: String) -> &mut Self {
        self.record_flag = Some(record_flag);
        self
    }

    pub fn set_record_num(&mut self, record_num: usize) -> &mut Self {
        self.record_num = Some(record_num);
        self
    }
    pub fn set_record_remark(&mut self, record_remark: String) -> &mut Self {
        self.record_remark = Some(record_remark);
        self
    }

    pub fn set_record_create_time(&mut self, record_create_time: String) -> &mut Self {
        self.record_create_time = Some(record_create_time);
        self
    }
    pub fn set_record_create_user(&mut self, record_create_user: String) -> &mut Self {
        self.record_create_user = Some(record_create_user);
        self
    }

    pub fn set_record_update_user(&mut self, record_update_user: String) -> &mut Self {
        self.record_update_user = Some(record_update_user);
        self
    }

    pub fn set_record_update_time(&mut self, record_update_time: String) -> &mut Self {
        self.record_update_time = Some(record_update_time);
        self
    }

    pub fn set_record_owner_user(&mut self, record_owner_user: String) -> &mut Self {
        self.record_owner_user = Some(record_owner_user);
        self
    }
    pub fn set_record_owner_org(&mut self, record_owner_org: String) -> &mut Self {
        self.record_owner_org = Some(record_owner_org);
        self
    }
    pub fn set_record_sign(&mut self, record_sign: String) -> &mut Self {
        self.record_sign = Some(record_sign);
        self
    }

    pub fn get_record_id(&self) -> Option<&String> {
        self.record_id.as_ref()
    }
    pub fn get_record_level(&self) -> Option<&String> {
        self.record_level.as_ref()
    }
    pub fn get_record_status(&self) -> Option<&String> {
        self.record_status.as_ref()
    }

    pub fn get_record_flag(&self) -> Option<&String> {
        self.record_flag.as_ref()
    }

    pub fn get_record_num(&self) -> Option<&usize> {
        self.record_num.as_ref()
    }
    pub fn get_record_remark(&self) -> Option<&String> {
        self.record_remark.as_ref()
    }

    pub fn get_record_create_time(&self) -> Option<&String> {
        self.record_create_time.as_ref()
    }
    pub fn get_record_create_user(&self) -> Option<&String> {
        self.record_create_user.as_ref()
    }

    pub fn get_record_update_user(&self) -> Option<&String> {
        self.record_update_user.as_ref()
    }

    pub fn get_record_update_time(&self) -> Option<&String> {
        self.record_update_time.as_ref()
    }

    pub fn get_record_owner_user(&self) -> Option<&String> {
        self.record_owner_user.as_ref()
    }
    pub fn get_record_owner_org(&self) -> Option<&String> {
        self.record_owner_org.as_ref()
    }
    pub fn get_record_sign(&self) -> Option<&String> {
        self.record_sign.as_ref()
    }
}

impl From<&BmbpBaseModel> for BmbpValue {
    fn from(value: &BmbpBaseModel) -> Self {
        let bmbp_hash_map = BmbpHashMap::from(value);
        BmbpValue::Map(bmbp_hash_map)
    }
}

impl From<&BmbpBaseModel> for BmbpHashMap {
    fn from(model: &BmbpBaseModel) -> Self {
        let mut bmbp_map = BmbpHashMap::new();
        bmbp_map.insert(
            "record_id".to_string(),
            BmbpValue::from(model.get_record_id()),
        );
        bmbp_map.insert(
            "record_level".to_string(),
            BmbpValue::from(model.get_record_level()),
        );
        bmbp_map.insert(
            "record_status".to_string(),
            BmbpValue::from(model.get_record_status()),
        );
        bmbp_map.insert(
            "record_flag".to_string(),
            BmbpValue::from(model.get_record_flag()),
        );
        bmbp_map.insert(
            "record_num".to_string(),
            BmbpValue::from(model.get_record_num()),
        );
        bmbp_map.insert(
            "record_remark".to_string(),
            BmbpValue::from(model.get_record_remark()),
        );
        bmbp_map.insert(
            "record_create_time".to_string(),
            BmbpValue::from(model.get_record_create_time()),
        );
        bmbp_map.insert(
            "record_create_user".to_string(),
            BmbpValue::from(model.get_record_create_user()),
        );

        bmbp_map.insert(
            "record_update_time".to_string(),
            BmbpValue::from(model.get_record_update_time()),
        );
        bmbp_map.insert(
            "record_update_user".to_string(),
            BmbpValue::from(model.get_record_update_user()),
        );
        bmbp_map.insert(
            "record_owner_user".to_string(),
            BmbpValue::from(model.get_record_owner_user()),
        );
        bmbp_map.insert(
            "record_owner_org".to_string(),
            BmbpValue::from(model.get_record_owner_org()),
        );
        bmbp_map.insert(
            "record_sign".to_string(),
            BmbpValue::from(model.get_record_sign()),
        );
        bmbp_map
    }
}
