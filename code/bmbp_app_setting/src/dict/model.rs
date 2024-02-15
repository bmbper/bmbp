use serde::{Deserialize, Serialize};
use bmbp_app_common::{BmbpPageParam};
use bmbp_rdbc::{RdbcTableModel, RdbcTreeTableRow};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictQueryParams {
    data_id: Option<String>,
    parent_code: Option<String>,
    show_level: Option<String>,
}
impl DictQueryParams{
    pub fn new() -> Self {
        DictQueryParams {
            data_id: None,
            parent_code: None,
            show_level: None,
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpDictType {
    Inner = 0,
    Custom = 1,
}

impl Default for BmbpDictType {
    fn default() -> Self {
        BmbpDictType::Custom
    }
}

impl RdbcTableModel for BmbpSettingDict {
    fn get_table_name() -> String {
        "BMBP_SETTING_DICT".to_string()
    }
    fn get_table_fields() -> Vec<String> {
        vec!["dict_alias".to_string(), "dict_value".to_string(), "dict_type".to_string()]
    }
}

pub type BmbpSettingDictOrmModel = RdbcTreeTableRow<BmbpSettingDict>;