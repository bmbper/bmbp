use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::RwLock;

pub const BMBP_TREE_ROOT_NODE: &str = "#";

/// RdbcTree 定义树型抽象
pub trait BmbpTree<T>
where
    T: BmbpTree<T>,
{
    fn get_code(&self) -> &String;
    fn set_code(&mut self, code: String) -> &mut Self;
    fn get_parent_code(&self) -> &String;
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self;
    fn get_children(&self) -> &[T];
    fn get_children_mut(&mut self) -> &mut [T];
    fn set_children(&mut self, children: Vec<T>) -> &mut Self;
    fn get_order(&self) -> u64 {
        0u64
    }
}
#[derive(Debug)]
struct RdbcTreeNodeRef<'a, T>
where
    T: BmbpTree<T> + Clone,
{
    ref_parent: RwLock<Option<&'a T>>,
    ref_node: Option<&'a T>,
    ref_children: RwLock<Vec<&'a RdbcTreeNodeRef<'a, T>>>,
}
pub struct BmbpTreeUtil;
impl BmbpTreeUtil {
    pub fn build<T>(tree_node_vec: Vec<T>) -> Vec<T>
    where
        T: BmbpTree<T> + Debug + Clone + Sync + Send + Clone + Default,
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

    fn build_tree_node_ref_map<T>(tree_node_slice: &[T]) -> HashMap<String, RdbcTreeNodeRef<T>>
    where
        T: BmbpTree<T> + Debug + Clone + Sync + Send + Clone + Default,
    {
        let mut tree_node_ref_map = HashMap::new();

        for tree_node in tree_node_slice {
            let tree_node_id = tree_node.get_code().clone();
            let tree_node_ref = RdbcTreeNodeRef {
                ref_node: Some(tree_node),
                ref_parent: RwLock::new(None),
                ref_children: RwLock::new(vec![]),
            };
            tree_node_ref_map.insert(tree_node_id, tree_node_ref);
        }
        tree_node_ref_map
    }

    fn build_tree_node_ref_to_parent<'a, T>(
        tree_node_slice: &[T],
        tree_node_ref_map: &'a HashMap<String, RdbcTreeNodeRef<'a, T>>,
    ) where
        T: BmbpTree<T> + Debug + Clone + Sync + Send + Clone + Default,
    {
        for tree_node in tree_node_slice {
            if tree_node_ref_map.contains_key(tree_node.get_parent_code()) {
                let current_ref = tree_node_ref_map.get(tree_node.get_code()).unwrap();
                let parent_ref = tree_node_ref_map.get(tree_node.get_parent_code()).unwrap();
                *current_ref.ref_parent.write().unwrap() = Some(parent_ref.ref_node.unwrap());
                parent_ref.ref_children.write().unwrap().push(current_ref);
            }
        }
    }

    fn build_tree_node_from_ref<T>(tree_node_ref_slice: &[&RdbcTreeNodeRef<T>]) -> Vec<T>
    where
        T: BmbpTree<T> + Debug + Clone + Sync + Send + Clone + Default,
    {
        let mut tree_node_vec = vec![];
        for tree_node_ref in tree_node_ref_slice {
            let mut tree_node = tree_node_ref.ref_node.unwrap().clone();
            let tree_node_ref_children_reader = tree_node_ref.ref_children.read().unwrap();
            let tree_node_ref_children_slice = tree_node_ref_children_reader.as_slice();
            let children_tree_node_vec =
                Self::build_tree_node_from_ref(tree_node_ref_children_slice);
            tree_node.set_children(children_tree_node_vec);
            tree_node_vec.push(tree_node);
        }
        tree_node_vec
    }
    fn build_tree_root_ref_vec<'a, T>(
        tree_node_ref_map: &'a HashMap<String, RdbcTreeNodeRef<'a, T>>,
    ) -> Vec<&'a RdbcTreeNodeRef<'a, T>>
    where
        T: BmbpTree<T> + Debug + Clone + Sync + Send + Clone + Default,
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
