use serde::{Deserialize, Serialize};
use bmbp_app_common::{BmbpBaseModel, BmbpTreeModel};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpSettingDictExtProps {
    // 公共字段
    #[serde(flatten)]
    base: BmbpBaseModel,
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

pub type BmbpSettingDict = BmbpTreeModel<BmbpSettingDictExtProps>;

#[cfg(test)]
pub mod tests {
    use bmbp_app_common::{BmbpTreeModel, ROOT_TREE_NODE};
    use crate::dict::model::BmbpSettingDict;

    #[test]
    fn test_dict_tree() {
        let mut dict_list = vec![];
        for i in 1..4 {
            let mut root_dict = BmbpSettingDict::default();
            root_dict.set_code(format!("{}", i));
            root_dict.set_name(format!("{}", i));
            root_dict.set_parent_code(ROOT_TREE_NODE.to_string());
            for j in 1..3 {
                let mut second_dict = BmbpSettingDict::default();
                second_dict.set_code(format!("{}-{}", i, j));
                second_dict.set_name(format!("{}-{}", i, j));
                second_dict.set_parent_code(root_dict.get_code().to_string());
                for k in 1..3 {
                    let mut third_dict = BmbpSettingDict::default();
                    third_dict.set_code(format!("{}-{}-{}", i, j, k));
                    third_dict.set_name(format!("{}-{}-{}", i, j, k));
                    third_dict.set_parent_code(second_dict.get_code().to_string());
                    dict_list.push(third_dict);
                }
                dict_list.push(second_dict);
            }
            dict_list.push(root_dict);
        }
        let tree_dict = BmbpTreeModel::build_tree(dict_list, true);
        assert_eq!(tree_dict.len(), 3);
        println!("{}", serde_json::to_string_pretty(&tree_dict).unwrap())
    }
}