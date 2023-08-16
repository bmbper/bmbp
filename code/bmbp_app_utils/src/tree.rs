use std::fmt::Debug;
use std::{collections::HashMap, sync::RwLock};

use bmbp_app_common::{BmbpHashMap, BmbpTree, BmbpValue, ROOT_TREE_NODE};

struct TreeGenericNodeRef<'a, T>
where
    T: BmbpTree<T> + Clone + Send + Sync + Debug,
{
    ref_parent: RwLock<Option<&'a T>>,
    ref_node: Option<&'a T>,
    ref_children: RwLock<Vec<&'a TreeGenericNodeRef<'a, T>>>,
}
pub struct TreeGenericBuilder {}

impl TreeGenericBuilder {
    pub fn build<T>(tree_node_vec: Vec<T>) -> Vec<T>
    where
        T: BmbpTree<T> + Clone + Sync + Send + Clone + Debug,
    {
        // 节点集合，方便后期直接从这里面取值
        let tree_node_ref_map = Self::build_tree_node_ref_map(tree_node_vec.as_slice());

        // 关联上级
        Self::build_tree_node_ref_to_parent(tree_node_vec.as_slice(), &tree_node_ref_map);

        // 提取根节点关联关系
        let tree_root_node_ref_vec: Vec<&TreeGenericNodeRef<T>> =
            Self::build_tree_root_ref_vec(&tree_node_ref_map);

        // 获取树根节点
        let tree_node_vec = Self::build_tree_node_from_ref(tree_root_node_ref_vec.as_slice());
        tree_node_vec
    }

    fn build_tree_node_from_ref<T>(tree_node_ref_slice: &[&TreeGenericNodeRef<T>]) -> Vec<T>
    where
        T: BmbpTree<T> + Clone + Sync + Send + Debug,
    {
        let mut tree_node_vec = vec![];
        for tree_node_ref in tree_node_ref_slice {
            let mut tree_node = tree_node_ref.ref_node.unwrap().clone();
            let tree_node_ref_children_reader = tree_node_ref.ref_children.read().unwrap();
            let tree_node_ref_children_slice = tree_node_ref_children_reader.as_slice();
            let children_tree_node_vec =
                Self::build_tree_node_from_ref(tree_node_ref_children_slice);
            tree_node.set_tree_children(children_tree_node_vec);
            tree_node_vec.push(tree_node);
        }
        tree_node_vec
    }

    fn build_tree_node_ref_map<T>(tree_node_slice: &[T]) -> HashMap<String, TreeGenericNodeRef<T>>
    where
        T: BmbpTree<T> + Clone + Sync + Send + Debug,
    {
        let mut tree_node_ref_map = HashMap::new();

        for tree_node in tree_node_slice {
            if let Some(tree_code) = tree_node.get_tree_code() {
                let tree_node_ref = TreeGenericNodeRef {
                    ref_node: Some(tree_node),
                    ref_parent: RwLock::new(None),
                    ref_children: RwLock::new(vec![]),
                };
                tree_node_ref_map.insert(tree_code.to_string(), tree_node_ref);
            }
        }
        tree_node_ref_map
    }

    fn build_tree_node_ref_to_parent<'a, T>(
        tree_node_slice: &[T],
        tree_node_ref_map: &'a HashMap<String, TreeGenericNodeRef<'a, T>>,
    ) where
        T: BmbpTree<T> + Clone + Send + Sync + Debug,
    {
        for tree_node in tree_node_slice {
            // 缺失上级节点ID的节点ID，上级节点ID改为根节点ID
            if let Some(tree_node_parent_id_ref) = tree_node.get_tree_parent_code() {
                let mut tree_node_parent_id = tree_node_parent_id_ref.to_string();
                if tree_node_parent_id.is_empty() {
                    tree_node_parent_id = ROOT_TREE_NODE.to_string();
                }
                if tree_node_ref_map.contains_key(&tree_node_parent_id) {
                    if let Some(tree_id) = tree_node.get_tree_code() {
                        let current_ref = tree_node_ref_map.get(tree_id).unwrap();
                        let parent_ref = tree_node_ref_map.get(&tree_node_parent_id).unwrap();
                        *current_ref.ref_parent.write().unwrap() =
                            Some(parent_ref.ref_node.unwrap());
                        parent_ref.ref_children.write().unwrap().push(current_ref);
                    }
                }
            }
        }
    }

    fn build_tree_root_ref_vec<'a, T>(
        tree_node_ref_map: &'a HashMap<String, TreeGenericNodeRef<'a, T>>,
    ) -> Vec<&'a TreeGenericNodeRef<'a, T>>
    where
        T: BmbpTree<T> + Clone + Debug + Send + Sync,
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

pub struct TreeHashMapNodeRef<'a> {
    pub parent: RwLock<Option<&'a BmbpHashMap>>,
    pub data: &'a BmbpHashMap,
    pub children: RwLock<Vec<&'a TreeHashMapNodeRef<'a>>>,
}

pub struct HashMapTreeBuilder;
impl HashMapTreeBuilder {
    pub fn build_tree_by_raw_name(
        data_list: Vec<BmbpHashMap>,
        tree_name: &str,
    ) -> Vec<BmbpHashMap> {
        let tree_code: String = tree_name.clone().to_string() + "_code";
        let tree_parent: String = tree_name.clone().to_string() + "_parent_code";
        let tree_children: String = tree_name.clone().to_string() + "_children";
        Self::build_tree_by_code_parent_children(
            data_list,
            tree_code.as_str(),
            tree_parent.as_str(),
            tree_children.as_str(),
        )
    }

    pub fn build_tree_by_name(data_list: Vec<BmbpHashMap>, tree_name: &str) -> Vec<BmbpHashMap> {
        let tree_code = tree_name.clone().to_string() + "Code";
        let tree_parent = tree_name.clone().to_string() + "ParentCode";
        let tree_children = tree_name.clone().to_string() + "Children";
        Self::build_tree_by_code_parent_children(
            data_list,
            tree_code.as_str(),
            tree_parent.as_str(),
            tree_children.as_str(),
        )
    }
    pub fn build_tree_by_code_parent_children(
        data_list: Vec<BmbpHashMap>,
        tree_code: &str,
        tree_parent: &str,
        tree_children: &str,
    ) -> Vec<BmbpHashMap> {
        // 构建TreeNode关联列表
        let mut node_map = {
            let mut node_list: HashMap<String, TreeHashMapNodeRef> = HashMap::new();
            for item in data_list.as_slice() {
                let node_ref = TreeHashMapNodeRef {
                    parent: RwLock::new(None),
                    data: item,
                    children: RwLock::new(vec![]),
                };
                let tree_code_rs = item.get(tree_code);
                match tree_code_rs {
                    Some(code) => {
                        node_list.insert(code.to_string(), node_ref);
                    }
                    None => {}
                }
            }
            node_list
        };
        // 关联上下级关系
        for item in data_list.as_slice() {
            // 当前节点
            let current_node = {
                if let Some(tree_code) = item.get(tree_code) {
                    let tree_code_string = tree_code.to_string();
                    node_map.get(tree_code_string.as_str())
                } else {
                    None
                }
            };

            // 上级节点
            let parent_node = {
                if let Some(tree_parent_code) = item.get(tree_parent) {
                    let tree_parent_code_string = tree_parent_code.to_string();
                    Some(node_map.get(tree_parent_code_string.as_str()).unwrap())
                } else {
                    None
                }
            };

            // 当前节点、或上级节点不存在, 不在处理
            if current_node.is_none() || parent_node.is_none() {
                continue;
            }

            let current_node_ref = current_node.unwrap();
            let parent_node_ref = parent_node.unwrap();

            *current_node_ref.parent.write().unwrap() = Some(parent_node_ref.data);
            parent_node_ref
                .children
                .write()
                .unwrap()
                .push(current_node_ref);
        }
        // 提取父节点为空的数据
        let root_node = {
            let mut root_node_vec = vec![];
            for item in node_map.values() {
                if item.parent.read().unwrap().is_none() {
                    root_node_vec.push(item);
                }
            }
            root_node_vec
        };
        // 根节点中获取数据
        Self::parse_tree_node_data(tree_children, root_node.as_slice())
    }

    fn parse_tree_node_data(
        tree_children: &str,
        node_slice: &[&TreeHashMapNodeRef],
    ) -> Vec<BmbpHashMap> {
        let mut tree_node_vec = vec![];
        for item in node_slice {
            let mut data = item.data.clone();
            let child_data =
                Self::parse_tree_node_data(tree_children, item.children.read().unwrap().as_slice());
            data.insert(tree_children.to_string(), BmbpValue::from(child_data));
            tree_node_vec.push(data);
        }
        tree_node_vec
    }
}
