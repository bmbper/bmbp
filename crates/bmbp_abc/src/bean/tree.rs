use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

pub const BMBP_TREE_ROOT_NODE: &str = "#";
pub trait BmbpTreeModel<T>
where
    T: BmbpTreeModel<T>,
{
    fn tree_mut(&mut self) -> &mut BmbpTree<T>;
    fn tree(&self) -> &BmbpTree<T>;
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpTree<T>
where
    T: BmbpTreeModel<T>,
{
    pub parent_node_code: String,
    pub node_code: String,
    pub node_code_path: String,
    pub node_name: String,
    pub node_name_path: String,
    pub node_desc: String,
    pub node_type: String,
    pub node_grade: usize,
    pub node_order: usize,
    pub children: Vec<T>,
    pub data_id: String,
    pub data_status: String,
    pub data_flag: String,
    pub data_level: String,
    pub data_sign: String,
    pub data_owner_org_code: String,
    pub data_create_time: String,
    pub data_create_user: String,
    pub data_update_time: String,
    pub data_update_user: String,
}

pub enum BmbpTreeColumn {
    ParentNodeCode,
    NodeCode,
    NodeCodePath,
    NodeName,
    NodeNamePath,
    NodeDesc,
    NodeType,
    NodeOrder,
    DataId,
    DataStatus,
    DataFlag,
    DataLevel,
    DataSign,
    DataOwnerOrgCode,
    DataCreateTime,
    DataUpdateTime,
    DataUpdateUser,
}

/// RdbcTree 定义树型抽象
struct RdbcTreeNodeRef<'a, T>
where
    T: BmbpTreeModel<T> + Clone,
{
    ref_parent: RwLock<Option<&'a T>>,
    ref_node: Option<&'a T>,
    ref_children: RwLock<Vec<&'a RdbcTreeNodeRef<'a, T>>>,
}
pub struct BmbpTreeUtil;
impl BmbpTreeUtil {
    pub fn build_tree<T>(tree_node_vec: Vec<T>) -> Vec<T>
    where
        T: BmbpTreeModel<T> + Clone,
    {
        // 节点集合，方便后期直接从这里面取值
        let tree_node_ref_map = Self::build_tree_node_ref_map(tree_node_vec.as_slice());
        // 关联上级
        Self::build_tree_node_ref_to_parent(tree_node_vec.as_slice(), &tree_node_ref_map);

        // 提取根节点关联关系
        let tree_root_node_ref_vec: Vec<&RdbcTreeNodeRef<T>> =
            Self::build_tree_root_ref_vec(&tree_node_ref_map);

        // 获取树根节点
        let tree_node_vec = Self::build_tree_node_from_ref(tree_root_node_ref_vec.as_slice());
        tree_node_vec
    }

    fn build_tree_node_from_ref<T>(tree_node_ref_slice: &[&RdbcTreeNodeRef<T>]) -> Vec<T>
    where
        T: BmbpTreeModel<T> + Clone,
    {
        let mut tree_node_vec = vec![];
        for tree_node_ref in tree_node_ref_slice {
            let mut tree_node = tree_node_ref.ref_node.unwrap().clone();
            let tree_node_ref_children_reader = tree_node_ref.ref_children.read().unwrap();
            let tree_node_ref_children_slice = tree_node_ref_children_reader.as_slice();
            let children_tree_node_vec =
                Self::build_tree_node_from_ref(tree_node_ref_children_slice);
            tree_node.tree_mut().children = children_tree_node_vec;
            tree_node_vec.push(tree_node);
        }
        tree_node_vec.sort_by(|a, b| a.tree().node_order.cmp(&b.tree().node_order));
        tree_node_vec
    }

    fn build_tree_node_ref_map<T>(tree_node_slice: &[T]) -> HashMap<String, RdbcTreeNodeRef<T>>
    where
        T: BmbpTreeModel<T> + Clone,
    {
        let mut tree_node_ref_map = HashMap::new();

        for tree_node in tree_node_slice {
            let tree_node_ref = RdbcTreeNodeRef {
                ref_node: Some(tree_node),
                ref_parent: RwLock::new(None),
                ref_children: RwLock::new(vec![]),
            };
            tree_node_ref_map.insert(tree_node.tree().node_code.clone(), tree_node_ref);
        }
        tree_node_ref_map
    }

    fn build_tree_node_ref_to_parent<'a, T>(
        tree_node_slice: &[T],
        tree_node_ref_map: &'a HashMap<String, RdbcTreeNodeRef<'a, T>>,
    ) where
        T: BmbpTreeModel<T> + Clone,
    {
        for tree_node in tree_node_slice {
            // 缺失上级节点ID的节点ID，上级节点ID改为根节点ID
            let mut tree_node_parent_id = tree_node.tree().parent_node_code.clone();
            if tree_node_parent_id.is_empty() {
                tree_node_parent_id = BMBP_TREE_ROOT_NODE.to_string();
            }
            if tree_node_ref_map.contains_key(&tree_node_parent_id) {
                let tree_id = tree_node.tree().node_code.clone();
                let current_ref = tree_node_ref_map.get(&tree_id).unwrap();
                let parent_ref = tree_node_ref_map.get(&tree_node_parent_id).unwrap();
                *current_ref.ref_parent.write().unwrap() = Some(parent_ref.ref_node.unwrap());
                parent_ref.ref_children.write().unwrap().push(current_ref);
            }
        }
    }

    fn build_tree_root_ref_vec<'a, T>(
        tree_node_ref_map: &'a HashMap<String, RdbcTreeNodeRef<'a, T>>,
    ) -> Vec<&'a RdbcTreeNodeRef<'a, T>>
    where
        T: BmbpTreeModel<T> + Clone,
    {
        let mut root_node_vec = vec![];
        for item in tree_node_ref_map.values() {
            if item.ref_parent.read().unwrap().is_none() {
                root_node_vec.push(item);
            }
        }
        root_node_vec
    }
}
