use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use bmbp_rdbc_sql::RdbcValue;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcPage<T> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
    page_size: usize,
    page_num: usize,
    total: usize,
    data: Option<Vec<T>>,
}

impl<T> RdbcPage<T> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
    pub fn new() -> Self {
        RdbcPage {
            page_size: 10,
            page_num: 1,
            total: 0,
            data: None,
        }
    }
    pub fn new_with_page(page_size: usize, page_num: usize) -> Self {
        RdbcPage {
            page_size,
            page_num,
            total: 0,
            data: None,
        }
    }
}

impl<T> RdbcPage<T> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {
    pub fn page_num(&self) -> &usize {
        &self.page_num
    }
    pub fn page_size(&self) -> &usize {
        &self.page_size
    }
    pub fn total(&self) -> &usize {
        &self.total
    }
    pub fn data(&self) -> &Option<Vec<T>> {
        &self.data
    }
    pub fn set_page_num(&mut self, page_num: usize) -> &mut Self {
        self.page_num = page_num;
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_total(&mut self, total: usize) -> &mut Self {
        self.total = total;
        self
    }
    pub fn set_data(&mut self, data: Option<Vec<T>>) -> &mut Self {
        self.data = data;
        self
    }
}

pub struct RdbcOrmRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}


/// RdbcTree 定义树型抽象
pub trait RdbcTree<T> {
}

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
pub trait RdbcModel {
    fn get_table_name() -> String;
    fn get_table_fields() -> Vec<String>;
    fn get_table_primary_key() -> String {
        "data_id".to_string()
    }
    fn get_table_union_primary_key() -> Vec<String> {
        return vec![];
    }
}

/// 定义数据库表结构实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcRowModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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

impl<T> RdbcModel for RdbcRowModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
    fn get_table_name() -> String {
        T::get_table_name()
    }

    fn get_table_fields() -> Vec<String> {
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
pub struct RdbcTreeRowModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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
    children: Vec<RdbcTreeRowModel<T>>,
    // 节点类型
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<bool>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcTreeRowModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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




