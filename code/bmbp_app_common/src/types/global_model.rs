use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use serde::Deserialize;
use serde::Serialize;

use crate::{BmbpHashMap, ROOT_TREE_NODE};
use crate::BmbpValue;

/// BmbpBaseModel 基础模型 用于存放公共字段
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
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

    pub fn get_table_columns() -> Vec<String> {
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

impl From<&BmbpBaseModel> for HashMap<String, BmbpValue> {
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


/// BmbpCurdModel
/// 用于标注表结构对应的抽象
pub trait BmbpCurdModel {
    fn get_table_name() -> String;
    fn get_table_columns() -> Vec<String>;
    fn get_table_alias() -> String {
        "t1".to_string()
    }
    fn get_table_primary_key() -> String {
        "record_id".to_string()
    }
}

/// BmbpOrmModel
/// 用于数据表结构对应的数据模型
/// 并声明公共字段
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrmModel<T> where T: Default + Debug + Clone + Serialize + BmbpCurdModel {
    /// 记录主键
    data_id: Option<String>,
    #[serde(flatten)]
    ext_props: T,
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
}

impl<T> BmbpOrmModel<T> where T: Default + Debug + Clone + Serialize + for<'a> Deserialize<'a> + BmbpCurdModel {
    pub fn new() -> BmbpOrmModel<T> {
        BmbpOrmModel::default()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn set_data_level(&mut self, data_level: String) -> &mut Self {
        self.data_level = Some(data_level);
        self
    }
    pub fn set_data_status(&mut self, data_status: String) -> &mut Self {
        self.data_status = Some(data_status);
        self
    }

    pub fn set_data_flag(&mut self, data_flag: String) -> &mut Self {
        self.data_flag = Some(data_flag);
        self
    }

    pub fn set_data_sort(&mut self, data_sort: usize) -> &mut Self {
        self.data_sort = Some(data_sort);
        self
    }
    pub fn set_data_remark(&mut self, data_remark: String) -> &mut Self {
        self.data_remark = Some(data_remark);
        self
    }

    pub fn set_data_create_time(&mut self, data_create_time: String) -> &mut Self {
        self.data_create_time = Some(data_create_time);
        self
    }
    pub fn set_data_create_user(&mut self, data_create_user: String) -> &mut Self {
        self.data_create_user = Some(data_create_user);
        self
    }

    pub fn set_data_update_user(&mut self, data_update_user: String) -> &mut Self {
        self.data_update_user = Some(data_update_user);
        self
    }

    pub fn set_data_update_time(&mut self, data_update_time: String) -> &mut Self {
        self.data_update_time = Some(data_update_time);
        self
    }

    pub fn set_data_owner_org(&mut self, data_owner_org: String) -> &mut Self {
        self.data_owner_org = Some(data_owner_org);
        self
    }
    pub fn set_data_sign(&mut self, data_sign: String) -> &mut Self {
        self.data_sign = Some(data_sign);
        self
    }

    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn get_data_level(&self) -> Option<&String> {
        self.data_level.as_ref()
    }
    pub fn get_data_status(&self) -> Option<&String> {
        self.data_status.as_ref()
    }

    pub fn get_data_flag(&self) -> Option<&String> {
        self.data_flag.as_ref()
    }

    pub fn get_data_sort(&self) -> Option<&usize> {
        self.data_sort.as_ref()
    }
    pub fn get_data_remark(&self) -> Option<&String> {
        self.data_remark.as_ref()
    }

    pub fn get_data_create_time(&self) -> Option<&String> {
        self.data_create_time.as_ref()
    }
    pub fn get_data_create_user(&self) -> Option<&String> {
        self.data_create_user.as_ref()
    }

    pub fn get_data_update_user(&self) -> Option<&String> {
        self.data_update_user.as_ref()
    }

    pub fn get_data_update_time(&self) -> Option<&String> {
        self.data_update_time.as_ref()
    }

    pub fn get_data_owner_org(&self) -> Option<&String> {
        self.data_owner_org.as_ref()
    }
    pub fn get_data_sign(&self) -> Option<&String> {
        self.data_sign.as_ref()
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

impl<T> BmbpCurdModel for BmbpOrmModel<T> where T: Default + Debug + Clone + Serialize + for<'a> Deserialize<'a> + BmbpCurdModel {
    fn get_table_name() -> String {
        T::get_table_name()
    }

    fn get_table_columns() -> Vec<String> {
        let mut ext_columns = T::get_table_columns();
        let base_columns = vec![
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
        ext_columns.extend_from_slice(base_columns.as_slice());
        ext_columns
    }

    fn get_table_alias() -> String {
        T::get_table_alias()
    }
    fn get_table_primary_key() -> String {
        T::get_table_primary_key()
    }
}


/// BmbpTreeModel
/// 用于标注树结构结点模型
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpTreeModel<T> where T: Default + Clone + Serialize + BmbpCurdModel {
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
    children: Vec<BmbpTreeModel<T>>,
    // 节点类型    
    node_type: Option<String>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<bool>,
    // 扩展属性
    #[serde(flatten)]
    node_props: Option<T>,
}

impl<T> BmbpCurdModel for BmbpTreeModel<T> where T: Default + Clone + Serialize + Deserialize<'static> + BmbpCurdModel {
    fn get_table_name() -> String {
        T::get_table_name()
    }

    fn get_table_columns() -> Vec<String> {
        let mut ext_columns = T::get_table_columns();
        let tree_columns = vec![
            "code".to_string(),
            "code_path".to_string(),
            "parent_code".to_string(),
            "name".to_string(),
            "name_path".to_string(),
            "node_type".to_string(),
            "node_sort".to_string(),
            "node_level".to_string(),
            "node_leaf".to_string(),
        ];
        ext_columns.extend_from_slice(tree_columns.as_slice());
        ext_columns
    }

    fn get_table_alias() -> String {
        T::get_table_alias()
    }
    fn get_table_primary_key() -> String {
        T::get_table_primary_key()
    }
}

impl<T> BmbpTreeModel<T> where T: Default + Clone + Serialize + BmbpCurdModel {
    pub fn new() -> Self {
        BmbpTreeModel {
            code: "".to_string(),
            code_path: "".to_string(),
            parent_code: "".to_string(),
            name: "".to_string(),
            name_path: "".to_string(),
            children: vec![],
            node_type: None,
            node_level: None,
            node_leaf: None,
            node_props: None,
        }
    }
    pub fn set_code(&mut self, code: String) -> &mut Self {
        self.code = code;
        self
    }
    pub fn set_code_path(&mut self, code_path: String) -> &mut Self {
        self.code_path = code_path;
        self
    }
    pub fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = parent_code;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn set_name_path(&mut self, name_path: String) -> &mut Self {
        self.name_path = name_path;
        self
    }
    pub fn set_children(&mut self, children: Vec<BmbpTreeModel<T>>) -> &mut Self {
        self.children = children;
        self
    }
    pub fn set_node_type(&mut self, node_type: String) -> &mut Self {
        self.node_type = Some(node_type);
        self
    }
    pub fn set_node_level(&mut self, node_level: usize) -> &mut Self {
        self.node_level = Some(node_level);
        self
    }
    pub fn set_node_leaf(&mut self, node_leaf: bool) -> &mut Self {
        self.node_leaf = Some(node_leaf);
        self
    }
    pub fn set_node_props_opt(&mut self, ext_props: Option<T>) -> &mut Self {
        self.node_props = ext_props;
        self
    }
    pub fn set_node_props(&mut self, ext_props: T) -> &mut Self {
        self.node_props = Some(ext_props);
        self
    }
    pub fn get_code(&self) -> &String {
        &self.code
    }
    pub fn get_code_path(&self) -> &String {
        &self.code_path
    }
    pub fn get_parent_code(&self) -> &String {
        &self.parent_code
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_name_path(&self) -> &String {
        &self.name_path
    }

    pub fn get_children(&mut self) -> &Vec<BmbpTreeModel<T>> {
        self.children.as_ref()
    }
    pub fn get_children_mut(&mut self) -> &mut Vec<BmbpTreeModel<T>> {
        self.children.as_mut()
    }
    pub fn get_children_slice(&self) -> &[BmbpTreeModel<T>] {
        self.children.as_slice()
    }
    pub fn get_children_mut_slice(&mut self) -> &mut [BmbpTreeModel<T>] {
        self.children.as_mut_slice()
    }

    pub fn get_node_type(&self) -> Option<&String> {
        self.node_type.as_ref()
    }

    pub fn get_node_level(&self) -> Option<&usize> {
        self.node_level.as_ref()
    }
    pub fn get_node_leaf(&self) -> Option<&bool> {
        self.node_leaf.as_ref()
    }
    pub fn get_node_props(&self) -> Option<&T> {
        self.node_props.as_ref()
    }
    pub fn get_node_props_mut(&mut self) -> Option<&mut T> {
        self.node_props.as_mut()
    }
}

/// 树型结构节点引用
struct BmbpTreeModelRef<'a, T> where T: Default + Clone + Serialize + BmbpCurdModel + 'a {
    code: &'a String,
    parent_code: &'a String,
    node: &'a BmbpTreeModel<T>,
    children: RwLock<Vec<Arc<BmbpTreeModelRef<'a, T>>>>,
}

impl<T> BmbpTreeModel<T> where T: Default + Clone + Serialize + BmbpCurdModel {
    fn build_tree_ref<'a>(node_list: &'a [BmbpTreeModel<T>]) -> HashMap<&'a String, Arc<BmbpTreeModelRef<T>>> {
        // 递归终结条件， 当传入的节点列表为空时，返回空的引用映射
        let mut ref_map = HashMap::new();
        if node_list.is_empty() {
            return ref_map;
        }
        for node_ref in node_list {
            let children = node_ref.get_children_slice();
            let child_ref_map = Self::build_tree_ref(children);
            ref_map.extend(child_ref_map);
            let node_ref = BmbpTreeModelRef {
                code: node_ref.get_code(),
                parent_code: node_ref.get_parent_code(),
                node: node_ref,
                children: RwLock::new(vec![]),
            };
            let arc_node = Arc::new(node_ref);
            ref_map.insert(&arc_node.code, arc_node.clone());
        }
        return ref_map;
    }

    fn build_tree_data(node_ref_list: &[Arc<BmbpTreeModelRef<T>>]) -> Vec<BmbpTreeModel<T>> {
        let mut data_list = vec![];
        if node_ref_list.is_empty() {
            return data_list;
        }
        node_ref_list.into_iter().for_each(|node_ref| {
            let children_node_ref = node_ref.children.read().unwrap();
            let children_data = Self::build_tree_data(children_node_ref.as_slice());
            let mut current_data = node_ref.node.clone();
            current_data.set_children(children_data);
            data_list.push(current_data);
        });
        data_list
    }
    /// build_tree
    /// has_spurious: 是否包含孤立节点
    pub fn build_tree(node_list: Vec<BmbpTreeModel<T>>, _has_spurious: bool) -> Vec<BmbpTreeModel<T>> {
        // 集合
        let mut node_ref_map: HashMap<&String, Arc<BmbpTreeModelRef<T>>> = Self::build_tree_ref(node_list.as_slice());

        // 拼接树型关系
        for node_ref_key in node_ref_map.keys() {
            let node = node_ref_map.get(node_ref_key).unwrap();
            let arc_node = (*node).clone();
            let parent_code = arc_node.parent_code;
            if node_ref_map.contains_key(parent_code) {
                let parent_node_ref = node_ref_map.get(parent_code).unwrap();
                parent_node_ref.children.write().unwrap().push(arc_node.clone());
            }
        }

        // 提取根节点
        let mut root_node_ref = vec![];
        node_ref_map.values().for_each(|node_ref| {
            let parent_code = node_ref.parent_code;
            if parent_code == ROOT_TREE_NODE || parent_code.is_empty() {
                let arc_node_ref = node_ref.clone();
                root_node_ref.push(arc_node_ref.clone());
            }
        });
        // 根据根节点关联，获取实际数据
        return Self::build_tree_data(root_node_ref.as_slice());
    }
    /// build_tree_with_spurious 包含孤立节点
    pub fn build_tree_with_spurious(node_list: Vec<BmbpTreeModel<T>>) -> Vec<BmbpTreeModel<T>> {
        Self::build_tree(node_list, true)
    }
    /// build_tree_without_spurious 不包含孤立节点
    pub fn build_tree_without_spurious(node_list: Vec<BmbpTreeModel<T>>) -> Vec<BmbpTreeModel<T>> {
        Self::build_tree(node_list, false)
    }
}

