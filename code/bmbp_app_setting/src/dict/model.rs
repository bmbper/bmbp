use serde::{Deserialize, Serialize};
use bmbp_app_common::{BmbpCurdModel, BmbpOrmModel, BmbpTreeModel, PageParams};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictQueryParams {
    pub(crate) record_id: Option<String>,
    pub(crate) parent_code: Option<String>,
    pub(crate) show_level: Option<String>,
}

type DictPageQueryParams = PageParams<DictQueryParams>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpSettingDict {
    // 字典别名
    dict_alise: String,
    // 字典值
    dict_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum BmbpDictType {
    Group = 0,
    Item = 1,
}

impl Default for BmbpDictType {
    fn default() -> Self {
        BmbpDictType::Group
    }
}

impl BmbpCurdModel for BmbpSettingDict {
    fn get_table_name() -> String {
        "BMBP_SETTING_DICT".to_string()
    }
    fn get_table_columns() -> Vec<String> {
        vec!["dict_alias".to_string(), "dict_value".to_string()]
    }
}

pub type BmbpSettingDictOrmModel = BmbpOrmModel<BmbpSettingDict>;
pub type BmbpSettingDictOrmTreeModel = BmbpTreeModel<BmbpSettingDictOrmModel>;
