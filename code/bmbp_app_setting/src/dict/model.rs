use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use bmbp_app_common::BmbpPageParam;
use bmbp_rdbc_orm::{BmbpOrmRdbcTree, RdbcModel, RdbcOrmRow};

use crate::dict::model::BmbpDictType::{Custom, Inner};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictQueryParams {
    data_id: Option<String>,
    parent_code: Option<String>,
    show_level: Option<usize>,
}
#[allow(dead_code)]
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
    dict_alias: Option<String>,
    // 字典值
    dict_value: Option<String>,
    // 字典类型
    dict_type: Option<BmbpDictType>,
}
#[allow(dead_code)]
impl BmbpSettingDict {
    pub fn new() -> Self {
        BmbpSettingDict {
            dict_alias: None,
            dict_value: None,
            dict_type: None,
        }
    }
    pub fn get_dict_alias(&self) -> Option<&String> {
        self.dict_alias.as_ref()
    }
    pub fn set_dict_alias(&mut self, dict_alias: String) -> &mut Self {
        self.dict_alias = Some(dict_alias);
        self
    }
    pub fn get_dict_value(&self) -> Option<&String> {
        self.dict_value.as_ref()
    }
    pub fn set_dict_value(&mut self, dict_value: String) -> &mut Self {
        self.dict_value = Some(dict_value);
        self
    }
    pub fn get_dict_type(&self) -> Option<&BmbpDictType> {
        self.dict_type.as_ref()
    }
    pub fn set_dict_type(&mut self, dict_type: BmbpDictType) -> &mut Self {
        self.dict_type = Some(dict_type);
        self
    }
}

impl From<RdbcOrmRow> for BmbpSettingDict {
    fn from(row: RdbcOrmRow) -> Self {
        let mut dict = BmbpSettingDict::default();
        if let Some(data) = row.get_data().get("dict_alias") {
            dict.set_dict_alias(data.to_string());
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

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpDictType {
    Inner = 0,
    Custom = 1,
}

impl BmbpDictType {
    pub fn value_of(data: String) -> Option<Self> {
        match data.as_str() {
            "0" => Some(Inner),
            "1" => Some(Custom),
            _ => None,
        }
    }
    pub fn value(&self) -> String {
        match self {
            Inner => "0".to_string(),
            Custom => "1".to_string(),
        }
    }
}

impl Default for BmbpDictType {
    fn default() -> Self {
        Custom
    }
}
impl Display for BmbpDictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Inner => "0".to_string(),
            Custom => "1".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl RdbcModel for BmbpSettingDict {
    fn get_table_name() -> String {
        "BMBP_SETTING_DICT".to_string()
    }
    fn get_table_fields() -> Vec<String> {
        vec![
            "dict_alias".to_string(),
            "dict_value".to_string(),
            "dict_type".to_string(),
        ]
    }
}

pub type BmbpSettingDictOrmModel = BmbpOrmRdbcTree<BmbpSettingDict>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpComboVo {
    label: String,
    value: String,
    children: Vec<BmbpComboVo>,
}

impl BmbpComboVo {
    pub fn new() -> Self {
        BmbpComboVo {
            label: "".to_string(),
            value: "".to_string(),
            children: vec![],
        }
    }
    pub fn get_label(&self) -> &String {
        &self.label
    }
    pub fn set_label(&mut self, label: String) -> &mut Self {
        self.label = label;
        self
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }
    pub fn get_children(&self) -> &Vec<BmbpComboVo> {
        &self.children
    }
    pub fn set_children(&mut self, children: Vec<BmbpComboVo>) -> &mut Self {
        self.children = children;
        self
    }
    pub fn add_child(&mut self, child: BmbpComboVo) -> &mut Self {
        self.children.push(child);
        self
    }
}
