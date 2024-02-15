use std::fmt::Debug;
use serde::{Deserialize, Serialize};

/// RdbcTree 定义树型抽象
pub trait RdbcTree {}

/// RdbcTreeNode 定义实际树节点
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcTreeNode<T> where T: Default + Debug + Clone + Serialize {
    // 节点编码
    code: String,
    // 节点路径编码
    code_path: String,
    // 父节点编码
    parent_code: String,
    // 节点名称
    name: String,
    // 节点路径名称
    name_path: String,
    // 子节点
    children: Vec<RdbcTreeNode<T>>,
    // 节点类型
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<bool>,
    #[serde(flatten)]
    ext_props: T,
}

/// RdbcTableModel 数据库表结构抽象
pub trait RdbcTableModel {
    fn get_table_name() -> String;
    fn get_table_fields() -> Vec<String>;
}

/// RdbcTablePrimaryKey 数据库表主键抽象
pub trait RdbcTablePrimaryKey {
    fn get_table_primary_key() -> String;
    fn get_table_union_primary_key() -> Vec<String> {
        return vec![];
    }
}

/// 定义数据库表结构实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    /// 记录主键
    data_id: Option<String>,
    /// 记录密级
    data_level: Option<String>,
    /// 记录状态
    data_status: Option<String>,
    /// 记录标识
    data_flag: Option<String>,
    /// 记录显示顺序
    data_sort: Option<usize>,
    /// 记录备注
    data_remark: Option<String>,
    /// 记录创建时间
    data_create_time: Option<String>,
    /// 记录创建人
    data_create_user: Option<String>,
    /// 记录更新时间
    data_update_time: Option<String>,
    /// 记录更新用户
    data_update_user: Option<String>,
    /// 记录所属组织
    data_owner_org: Option<String>,
    /// 记录防串改标识
    data_sign: Option<String>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcTablePrimaryKey for RdbcTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    fn get_table_primary_key() -> String {
        return "data_id".to_string();
    }
}

impl<T> RdbcTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    pub fn get_table_fields() -> Vec<String> {
        let mut row_fields = vec![
            "data_id".to_string(),
            "data_level".to_string(),
            "data_status".to_string(),
            "data_flag".to_string(),
            "data_sort".to_string(),
            "data_remark".to_string(),
            "data_create_time".to_string(),
            "data_create_user".to_string(),
            "data_update_time".to_string(),
            "data_update_user".to_string(),
            "data_owner_org".to_string(),
            "data_sign".to_string(),
        ];
        let table_fields = T::get_table_fields();
        row_fields.extend_from_slice(table_fields.as_slice());
        row_fields
    }
}


/// /// 定义数据库表-树型结构实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcTreeTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    /// 记录主键
    data_id: Option<String>,
    /// 记录密级
    data_level: Option<String>,
    /// 记录状态
    data_status: Option<String>,
    /// 记录标识
    data_flag: Option<String>,
    /// 记录显示顺序
    data_sort: Option<usize>,
    /// 记录备注
    data_remark: Option<String>,
    /// 记录创建时间
    data_create_time: Option<String>,
    /// 记录创建人
    data_create_user: Option<String>,
    /// 记录更新时间
    data_update_time: Option<String>,
    /// 记录更新用户
    data_update_user: Option<String>,
    /// 记录所属组织
    data_owner_org: Option<String>,
    /// 记录防串改标识
    data_sign: Option<String>,
    // 节点编码
    code: String,
    // 节点路径编码
    code_path: String,
    // 父节点编码
    parent_code: String,
    // 节点名称
    name: String,
    // 节点路径名称
    name_path: String,
    // 子节点
    children: Vec<RdbcTreeTableRow<T>>,
    // 节点类型
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<bool>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcTablePrimaryKey for RdbcTreeTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    fn get_table_primary_key() -> String {
        return "data_id".to_string();
    }
}

impl<T> RdbcTreeTableRow<T> where T: Default + Debug + Clone + Serialize + RdbcTableModel {
    pub fn get_table_fields() -> Vec<String> {
        let mut row_fields = vec![
            "data_id".to_string(),
            "data_level".to_string(),
            "data_status".to_string(),
            "data_flag".to_string(),
            "data_sort".to_string(),
            "data_remark".to_string(),
            "data_create_time".to_string(),
            "data_create_user".to_string(),
            "data_update_time".to_string(),
            "data_update_user".to_string(),
            "data_owner_org".to_string(),
            "data_sign".to_string(),
            "code".to_string(),
            "code_path".to_string(),
            "parent_code".to_string(),
            "name".to_string(),
            "name_path".to_string(),
            "node_type".to_string(),
            "node_level".to_string(),
            "node_leaf".to_string(),
        ];
        let table_fields = T::get_table_fields();
        row_fields.extend_from_slice(table_fields.as_slice());
        row_fields
    }
}




