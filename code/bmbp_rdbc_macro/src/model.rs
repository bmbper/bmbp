use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use bmbp_rdbc_sql::RdbcValue;


/// RdbcModel 定义数据库表标记
pub trait RdbcModel {
    fn get_table_name() -> String;
    fn get_table_fields() -> Vec<String>;
    fn get_table_primary_key() -> String {
        "".to_string()
    }
    fn get_table_union_primary_key() -> Vec<String> {
        return vec![];
    }
}

/// 定义平台提供的数据库表实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRdbcModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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

impl<T> BmbpRdbcModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn get_data_level(&self) -> Option<&String> {
        self.data_level.as_ref()
    }
    pub fn set_data_level(&mut self, data_level: String) -> &mut Self {
        self.data_level = Some(data_level);
        self
    }
    pub fn get_data_status(&self) -> Option<&String> {
        self.data_status.as_ref()
    }
    pub fn set_data_status(&mut self, data_status: String) -> &mut Self {
        self.data_status = Some(data_status);
        self
    }
    pub fn get_data_flag(&self) -> Option<&String> {
        self.data_flag.as_ref()
    }
    pub fn set_data_flag(&mut self, data_flag: String) -> &mut Self {
        self.data_flag = Some(data_flag);
        self
    }
    pub fn get_data_sort(&self) -> Option<&usize> {
        self.data_sort.as_ref()
    }
    pub fn set_data_sort(&mut self, data_sort: usize) -> &mut Self {
        self.data_sort = Some(data_sort);
        self
    }
    pub fn get_data_remark(&self) -> Option<&String> {
        self.data_remark.as_ref()
    }
    pub fn set_data_remark(&mut self, data_remark: String) -> &mut Self {
        self.data_remark = Some(data_remark);
        self
    }
    pub fn get_data_create_time(&self) -> Option<&String> {
        self.data_create_time.as_ref()
    }
    pub fn set_data_create_time(&mut self, data_create_time: String) -> &mut Self {
        self.data_create_time = Some(data_create_time);
        self
    }
    pub fn get_data_create_user(&self) -> Option<&String> {
        self.data_create_user.as_ref()
    }
    pub fn set_data_create_user(&mut self, data_create_user: String) -> &mut Self {
        self.data_create_user = Some(data_create_user);
        self
    }
    pub fn get_data_update_time(&self) -> Option<&String> {
        self.data_update_time.as_ref()
    }
    pub fn set_data_update_time(&mut self, data_update_time: String) -> &mut Self {
        self.data_update_time = Some(data_update_time);
        self
    }
    pub fn get_data_update_user(&self) -> Option<&String> {
        self.data_update_user.as_ref()
    }
    pub fn set_data_update_user(&mut self, data_update_user: String) -> &mut Self {
        self.data_update_user = Some(data_update_user);
        self
    }
    pub fn get_data_owner_org(&self) -> Option<&String> {
        self.data_owner_org.as_ref()
    }
    pub fn set_data_owner_org(&mut self, data_owner_org: String) -> &mut Self {
        self.data_owner_org = Some(data_owner_org);
        self
    }
    pub fn get_data_sign(&self) -> Option<&String> {
        self.data_sign.as_ref()
    }
    pub fn set_data_sign(&mut self, data_sign: String) -> &mut Self {
        self.data_sign = Some(data_sign);
        self
    }
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
}

impl<T> RdbcModel for BmbpRdbcModel<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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

    fn get_table_primary_key() -> String {
        let pri = T::get_table_primary_key();
        if pri != "" {
            pri
        } else {
            "data_id".to_string()
        }
    }
}

/// RdbcTree 定义树型抽象
pub trait RdbcTree<T> where T: RdbcTree<T> {
    fn get_code(&self) -> &String;
    fn set_code(&mut self, code: String) -> &mut Self;
    fn get_code_path(&self) -> &String;
    fn set_code_path(&mut self, code_path: String) -> &mut Self;
    fn get_parent_code(&self) -> &String;
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self;
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String) -> &mut Self;
    fn get_name_path(&self) -> &String;
    fn set_name_path(&mut self, name_path: String) -> &mut Self;
    fn get_children(&self) -> &Vec<T>;
    fn get_children_mut(&mut self) -> &mut Vec<T>;
    fn set_children(&mut self, children: Vec<T>) -> &mut Self;
    fn get_node_type(&self) -> &Option<String>;
    fn set_node_type(&mut self, node_type: Option<String>) -> &mut Self;
    fn get_node_level(&self) -> &Option<usize>;
    fn set_node_level(&mut self, node_level: Option<usize>) -> &mut Self;
    fn get_node_leaf(&self) -> &Option<usize>;
    fn set_node_leaf(&mut self, node_leaf: Option<usize>) -> &mut Self;
}

/// BmbpRdbcTree 定义平台提供的树节点
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRdbcTree<T> where T: Default + Debug + Clone + Serialize {
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
    children: Vec<BmbpRdbcTree<T>>,
    // 节点类型
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<usize>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcTree<BmbpRdbcTree<T>> for BmbpRdbcTree<T> where T: Default + Debug + Clone + Serialize {
    fn get_code(&self) -> &String {
        &self.code
    }
    fn set_code(&mut self, code: String) -> &mut Self {
        self.code = code;
        self
    }
    fn get_code_path(&self) -> &String {
        &self.code_path
    }
    fn set_code_path(&mut self, code_path: String) -> &mut Self {
        self.code_path = code_path;
        self
    }
    fn get_parent_code(&self) -> &String {
        &self.parent_code
    }
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = parent_code;
        self
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    fn get_name_path(&self) -> &String {
        &self.name_path
    }
    fn set_name_path(&mut self, name_path: String) -> &mut Self {
        self.name_path = name_path;
        self
    }
    fn get_children(&self) -> &Vec<BmbpRdbcTree<T>> {
        &self.children
    }
    fn get_children_mut(&mut self) -> &mut Vec<BmbpRdbcTree<T>> {
        &mut self.children
    }
    fn set_children(&mut self, children: Vec<BmbpRdbcTree<T>>) -> &mut Self {
        self.children = children;
        self
    }
    fn get_node_type(&self) -> &Option<String> {
        &self.node_type
    }
    fn set_node_type(&mut self, node_type: Option<String>) -> &mut Self {
        self.node_type = node_type;
        self
    }
    fn get_node_level(&self) -> &Option<usize> {
        &self.node_level
    }
    fn set_node_level(&mut self, node_level: Option<usize>) -> &mut Self {
        self.node_level = node_level;
        self
    }
    fn get_node_leaf(&self) -> &Option<usize> {
        &self.node_leaf
    }
    fn set_node_leaf(&mut self, node_leaf: Option<usize>) -> &mut Self {
        self.node_leaf = node_leaf;
        self
    }
}

impl<T> BmbpRdbcTree<T> where T: Default + Debug + Clone + Serialize {
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
    pub fn get_ext_props_mut(&mut self) -> &mut T {
        &mut self.ext_props
    }
}

/// /// 定义数据库表-树型结构实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrmRdbcTree<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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
    children: Vec<BmbpOrmRdbcTree<T>>,
    // 节点类型
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<usize>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcModel for BmbpOrmRdbcTree<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
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

    fn get_table_primary_key() -> String {
        "data_id".to_string()
    }
}

impl<T> RdbcTree<BmbpOrmRdbcTree<T>> for BmbpOrmRdbcTree<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
    fn get_code(&self) -> &String {
        &self.code
    }
    fn set_code(&mut self, code: String) -> &mut Self {
        self.code = code;
        self
    }
    fn get_code_path(&self) -> &String {
        &self.code_path
    }
    fn set_code_path(&mut self, code_path: String) -> &mut Self {
        self.code_path = code_path;
        self
    }
    fn get_parent_code(&self) -> &String {
        &self.parent_code
    }
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = parent_code;
        self
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    fn get_name_path(&self) -> &String {
        &self.name_path
    }
    fn set_name_path(&mut self, name_path: String) -> &mut Self {
        self.name_path = name_path;
        self
    }
    fn get_children(&self) -> &Vec<BmbpOrmRdbcTree<T>> {
        &self.children
    }
    fn get_children_mut(&mut self) -> &mut Vec<BmbpOrmRdbcTree<T>> {
        &mut self.children
    }
    fn set_children(&mut self, children: Vec<BmbpOrmRdbcTree<T>>) -> &mut Self {
        self.children = children;
        self
    }
    fn get_node_type(&self) -> &Option<String> {
        &self.node_type
    }
    fn set_node_type(&mut self, node_type: Option<String>) -> &mut Self {
        self.node_type = node_type;
        self
    }
    fn get_node_level(&self) -> &Option<usize> {
        &self.node_level
    }
    fn set_node_level(&mut self, node_level: Option<usize>) -> &mut Self {
        self.node_level = node_level;
        self
    }
    fn get_node_leaf(&self) -> &Option<usize> {
        &self.node_leaf
    }
    fn set_node_leaf(&mut self, node_leaf: Option<usize>) -> &mut Self {
        self.node_leaf = node_leaf;
        self
    }
}

impl<T> BmbpOrmRdbcTree<T> where T: Default + Debug + Clone + Serialize + RdbcModel {
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn get_data_level(&self) -> Option<&String> {
        self.data_level.as_ref()
    }
    pub fn set_data_level(&mut self, data_level: String) -> &mut Self {
        self.data_level = Some(data_level);
        self
    }
    pub fn get_data_status(&self) -> Option<&String> {
        self.data_status.as_ref()
    }
    pub fn set_data_status(&mut self, data_status: String) -> &mut Self {
        self.data_status = Some(data_status);
        self
    }
    pub fn get_data_flag(&self) -> Option<&String> {
        self.data_flag.as_ref()
    }
    pub fn set_data_flag(&mut self, data_flag: String) -> &mut Self {
        self.data_flag = Some(data_flag);
        self
    }
    pub fn get_data_sort(&self) -> Option<&usize> {
        self.data_sort.as_ref()
    }
    pub fn set_data_sort(&mut self, data_sort: usize) -> &mut Self {
        self.data_sort = Some(data_sort);
        self
    }
    pub fn get_data_remark(&self) -> Option<&String> {
        self.data_remark.as_ref()
    }
    pub fn set_data_remark(&mut self, data_remark: String) -> &mut Self {
        self.data_remark = Some(data_remark);
        self
    }
    pub fn get_data_create_time(&self) -> Option<&String> {
        self.data_create_time.as_ref()
    }
    pub fn set_data_create_time(&mut self, data_create_time: String) -> &mut Self {
        self.data_create_time = Some(data_create_time);
        self
    }
    pub fn get_data_create_user(&self) -> Option<&String> {
        self.data_create_user.as_ref()
    }
    pub fn set_data_create_user(&mut self, data_create_user: String) -> &mut Self {
        self.data_create_user = Some(data_create_user);
        self
    }
    pub fn get_data_update_time(&self) -> Option<&String> {
        self.data_update_time.as_ref()
    }
    pub fn set_data_update_time(&mut self, data_update_time: String) -> &mut Self {
        self.data_update_time = Some(data_update_time);
        self
    }
    pub fn get_data_update_user(&self) -> Option<&String> {
        self.data_update_user.as_ref()
    }
    pub fn set_data_update_user(&mut self, data_update_user: String) -> &mut Self {
        self.data_update_user = Some(data_update_user);
        self
    }
    pub fn get_data_owner_org(&self) -> Option<&String> {
        self.data_owner_org.as_ref()
    }
    pub fn set_data_owner_org(&mut self, data_owner_org: String) -> &mut Self {
        self.data_owner_org = Some(data_owner_org);
        self
    }
    pub fn get_data_sign(&self) -> Option<&String> {
        self.data_sign.as_ref()
    }
    pub fn set_data_sign(&mut self, data_sign: String) -> &mut Self {
        self.data_sign = Some(data_sign);
        self
    }
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn get_ext_props_mut(&mut self) -> &mut T {
        &mut self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
}

/// 定义返回值类型
/// RdbcOrmRow 数据库查询结果 实现各个数据库的FromRow
/// RdbcPage 分页返回值
pub struct RdbcOrmRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}

impl RdbcOrmRow {
    pub fn new() -> Self {
        RdbcOrmRow {
            columns: vec![],
            data: HashMap::new(),
        }
    }
    pub fn get_columns(&self) -> &Vec<String> {
        &self.columns
    }
    pub fn get_columns_mut(&mut self) -> &mut Vec<String> {
        &mut self.columns
    }
    pub fn get_data(&self) -> &HashMap<String, RdbcValue> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut HashMap<String, RdbcValue> {
        &mut self.data
    }
}

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


impl<T> From<RdbcOrmRow> for BmbpRdbcModel<T> where T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize + RdbcModel {
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpRdbcModel::<T>::default();
        if let Some(data) = row.data.get("data_id") {
            model.set_data_id(data.get_string());
        }
        if let Some(data) = row.data.get("data_level") {
            model.set_data_level(data.get_string());
        }
        if let Some(data) = row.data.get("data_status") {
            model.set_data_status(data.get_string());
        }
        if let Some(data) = row.data.get("data_flag") {
            model.set_data_flag(data.get_string());
        }
        if let Some(data) = row.data.get("data_sort") {
            if let Some(v) = data.get_usize() {
                model.set_data_sort(v);
            }
        }
        if let Some(data) = row.data.get("data_remark") {
            model.set_data_remark(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_time") {
            model.set_data_create_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_user") {
            model.set_data_create_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_time") {
            model.set_data_update_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_user") {
            model.set_data_update_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_owner_org") {
            model.set_data_owner_org(data.get_string());
        }
        if let Some(data) = row.data.get("data_sign") {
            model.set_data_sign(data.get_string());
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
    }
}

impl<T> From<RdbcOrmRow> for BmbpRdbcTree<T> where T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize {
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpRdbcTree::<T>::default();
        if let Some(code) = row.data.get("code") {
            model.set_code(code.get_string());
        }
        if let Some(data) = row.data.get("code_path") {
            model.set_code_path(data.get_string());
        }
        if let Some(data) = row.data.get("parent_code") {
            model.set_parent_code(data.get_string());
        }
        if let Some(data) = row.data.get("name") {
            model.set_name(data.get_string());
        }
        if let Some(data) = row.data.get("name_path") {
            model.set_name_path(data.get_string());
        }
        if let Some(data) = row.data.get("node_type") {
            model.set_node_type(Some(data.get_string()));
        }
        if let Some(data) = row.data.get("node_level") {
            model.set_node_level(data.get_usize());
        }
        if let Some(data) = row.data.get("node_leaf") {
            model.set_node_leaf(data.get_usize());
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
    }
}

impl<T> From<RdbcOrmRow> for BmbpOrmRdbcTree<T> where T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize + RdbcModel {
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpOrmRdbcTree::<T>::default();
        if let Some(data) = row.data.get("data_id") {
            model.set_data_id(data.get_string());
        }
        if let Some(data) = row.data.get("data_level") {
            model.set_data_level(data.get_string());
        }
        if let Some(data) = row.data.get("data_status") {
            model.set_data_status(data.get_string());
        }
        if let Some(data) = row.data.get("data_flag") {
            model.set_data_flag(data.get_string());
        }
        if let Some(data) = row.data.get("data_sort") {
            if let Some(v) = data.get_usize() {
                model.set_data_sort(v);
            }
        }
        if let Some(data) = row.data.get("data_remark") {
            model.set_data_remark(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_time") {
            model.set_data_create_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_user") {
            model.set_data_create_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_time") {
            model.set_data_update_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_user") {
            model.set_data_update_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_owner_org") {
            model.set_data_owner_org(data.get_string());
        }
        if let Some(data) = row.data.get("data_sign") {
            model.set_data_sign(data.get_string());
        }
        if let Some(code) = row.data.get("code") {
            model.set_code(code.get_string());
        }
        if let Some(data) = row.data.get("code_path") {
            model.set_code_path(data.get_string());
        }
        if let Some(data) = row.data.get("parent_code") {
            model.set_parent_code(data.get_string());
        }
        if let Some(data) = row.data.get("name") {
            model.set_name(data.get_string());
        }
        if let Some(data) = row.data.get("name_path") {
            model.set_name_path(data.get_string());
        }
        if let Some(data) = row.data.get("node_type") {
            model.set_node_type(Some(data.get_string()));
        }
        if let Some(data) = row.data.get("node_level") {
            model.set_node_level(data.get_usize());
        }
        if let Some(data) = row.data.get("node_leaf") {
            if let Some(v) = data.get_usize() {
                model.set_node_leaf(Some(v));
            }
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
    }
}


impl From<Row> for RdbcOrmRow {
    fn from(row: Row) -> Self {
        let mut orm_row = RdbcOrmRow::new();
        let columns = row.columns();
        for col in columns {
            println!("===>{:#?}", col);
            let col_name = col.name().to_string();
            orm_row.get_columns_mut().push(col_name.clone());
            let col_type = col.type_().name().to_string();
            match col_type.as_str() {
                "text" => {
                    let value: String = row.get(col_name.as_str());
                    orm_row.get_data_mut().insert(col_name, RdbcValue::String(value));
                }
                _=>{

                }
            }
        }
        orm_row
    }
}

