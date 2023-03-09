use crate::dict::model::BmbpDictType::GROUP;
use bmbp_types::vo::BaseOrmModel;
use bmbp_types::BmbpBaseModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BmbpDictType {
    GROUP,
    ITEM,
    VALUE,
    CASCADE,
}
impl Default for BmbpDictType {
    fn default() -> Self {
        GROUP
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpDictModel {
    // 字典编码
    dict_code: String,
    // 上级字典编码
    parent_dict_code: String,
    // 标题
    dict_title: String,
    // 字典路径
    dict_path: String,
    // 字典值
    dict_value: String,
    // 惟一标识
    dict_key: String,
    // 字典类型
    dict_type: BmbpDictType,
    // 目标类型: dict_key
    target_dict: String,
    // 公共字段
    base: BmbpBaseModel,
}

impl BmbpDictModel {
    pub fn get_dict_code(&self) -> &String {
        &self.dict_code
    }
    pub fn get_parent_code(&self) -> &String {
        &self.parent_dict_code
    }
    pub fn get_dict_title(&self) -> &String {
        &self.dict_title
    }
    pub fn get_dict_value(&self) -> &String {
        &self.dict_value
    }
    pub fn get_dict_path(&self) -> &String {
        &self.dict_path
    }
    pub fn get_dict_key(&self) -> &String {
        &self.dict_key
    }
    pub fn get_dict_type(&self) -> &BmbpDictType {
        &self.dict_type
    }
}

impl BaseOrmModel for BmbpDictModel {
    fn get_base_vo(&self) -> &BmbpBaseModel {
        &self.base
    }
    fn get_mut_base_vo(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }
    fn set_base_vo(&mut self, vo: BmbpBaseModel) -> &mut Self {
        self.base = vo;
        self
    }
    fn vo_fields() -> Vec<String> {
        vec![
            "dict_code".to_string(),
            "parent_dict_code".to_string(),
            "dict_title".to_string(),
            "dict_path".to_string(),
            "dict_value".to_string(),
            "dict_type".to_string(),
            "dict_key".to_string(),
            "target_dict".to_string(),
        ]
    }
}
