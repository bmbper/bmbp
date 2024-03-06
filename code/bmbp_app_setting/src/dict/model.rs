use serde::{Deserialize, Serialize};
use bmbp_app_common::{BmbpPageParam};
use bmbp_rdbc_orm::{RdbcModel, BmbpOrmRdbcTree, RdbcOrmRow};
use crate::dict::model::BmbpDictType::{Custom, Inner};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictQueryParams {
    data_id: Option<String>,
    parent_code: Option<String>,
    show_level: Option<usize>,
}

impl DictQueryParams {
    pub fn new() -> Self {
        DictQueryParams {
            data_id: None,
            parent_code: None,
            show_level: None,
        }
    }
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn get_parent_code(&self) -> Option<&String> {
        self.parent_code.as_ref()
    }
    pub fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = Some(parent_code);
        self
    }

    pub fn get_show_level(&self) -> Option<&usize> {
        self.show_level.as_ref()
    }
    pub fn set_show_level(&mut self, show_level: usize) -> &mut Self {
        self.show_level = Some(show_level);
        self
    }
}

type DictPageQueryParams = BmbpPageParam<DictQueryParams>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpSettingDict {
    // 字典别名
    dict_alise: String,
    // 字典值
    dict_value: String,
    // 字典类型
    dict_type: BmbpDictType,
}

impl BmbpSettingDict {
    pub fn new() -> Self {
        BmbpSettingDict {
            dict_alise: "".to_string(),
            dict_value: "".to_string(),
            dict_type: Custom,
        }
    }
    pub fn get_dict_alise(&self) -> &String {
        &self.dict_alise
    }
    pub fn set_dict_alise(&mut self, dict_alise: String) -> &mut Self {
        self.dict_alise = dict_alise;
        self
    }
    pub fn get_dict_value(&self) -> &String {
        &self.dict_value
    }
    pub fn set_dict_value(&mut self, dict_value: String) -> &mut Self {
        self.dict_value = dict_value;
        self
    }
    pub fn get_dict_type(&self) -> &BmbpDictType {
        &self.dict_type
    }
    pub fn set_dict_type(&mut self, dict_type: BmbpDictType) -> &mut Self {
        self.dict_type = dict_type;
        self
    }
}

impl From<RdbcOrmRow> for BmbpSettingDict {
    fn from(row: RdbcOrmRow) -> Self {
        let mut dict = BmbpSettingDict::default();
        if let Some(data) = row.get_data().get("dict_alias") {
            dict.set_dict_alise(data.to_string());
        }
        if let Some(data) = row.get_data().get("dict_value") {
            dict.set_dict_value(data.to_string());
        }
        if let Some(data) = row.get_data().get("dict_type") {
            if let Some(dict_type) = BmbpDictType::value_of(data.get_string()) {
                dict.set_dict_type(dict_type);
            }
        }
        dict
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpDictType {
    Inner = 0,
    Custom = 1,
}

impl BmbpDictType {
    fn value_of(data: String) -> Option<Self> {
        match data.as_str() {
            "0" => {
                Some(Inner)
            }
            "1" => {
                Some(Custom)
            }
            _ => { None }
        }
    }
}

impl Default for BmbpDictType {
    fn default() -> Self {
        Custom
    }
}

impl RdbcModel for BmbpSettingDict {
    fn get_table_name() -> String {
        "BMBP_SETTING_DICT".to_string()
    }
    fn get_table_fields() -> Vec<String> {
        vec!["dict_alias".to_string(), "dict_value".to_string(), "dict_type".to_string()]
    }
}

pub type BmbpSettingDictOrmModel = BmbpOrmRdbcTree<BmbpSettingDict>;