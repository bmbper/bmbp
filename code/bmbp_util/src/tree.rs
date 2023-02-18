use std::fmt::Debug;
use std::{collections::HashMap, sync::RwLock};

use bmbp_types::{TreeNode, ROOT_TREE_NODE};

#[derive(Debug)]
struct TreeNodeRef<'a, T>
where
    T: TreeNode<T> + Clone + Send + Sync + Debug,
{
    ref_parent: RwLock<Option<&'a T>>,
    ref_node: Option<&'a T>,
    ref_children: RwLock<Vec<&'a TreeNodeRef<'a, T>>>,
}

pub struct TreeBuilder {}

impl TreeBuilder {
    pub fn build<T>(tree_node_vec: Vec<T>) -> Vec<T>
    where
        T: TreeNode<T> + Clone + Sync + Send + Clone + Debug,
    {
        // 节点集合，方便后期直接从这里面取值
        let tree_node_ref_map = Self::build_tree_node_ref_map(tree_node_vec.as_slice());

        // 关联上级
        Self::build_tree_node_ref_to_parent(tree_node_vec.as_slice(), &tree_node_ref_map);

        // 提取根节点关联关系
        let tree_root_node_ref_vec: Vec<&TreeNodeRef<T>> =
            Self::build_tree_root_ref_vec(&tree_node_ref_map);

        // 获取树根节点
        let tree_node_vec = Self::build_tree_node_from_ref(tree_root_node_ref_vec.as_slice());
        tree_node_vec
    }

    fn build_tree_node_from_ref<T>(tree_node_ref_slice: &[&TreeNodeRef<T>]) -> Vec<T>
    where
        T: TreeNode<T> + Clone + Sync + Send + Debug,
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

    fn build_tree_node_ref_map<T>(tree_node_slice: &[T]) -> HashMap<String, TreeNodeRef<T>>
    where
        T: TreeNode<T> + Clone + Sync + Send + Debug,
    {
        let mut tree_node_ref_map = HashMap::new();

        for tree_node in tree_node_slice {
            let tree_node_id = tree_node.node_id().clone();
            let tree_node_ref = TreeNodeRef {
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
        tree_node_ref_map: &'a HashMap<String, TreeNodeRef<'a, T>>,
    ) where
        T: TreeNode<T> + Clone + Send + Sync + Debug,
    {
        for tree_node in tree_node_slice {
            // 缺失上级节点ID的节点ID，上级节点ID改为根节点ID
            let mut tree_node_parent_id = tree_node.node_parent_id().clone();
            if tree_node_parent_id.is_empty() {
                tree_node_parent_id = ROOT_TREE_NODE.to_string();
            }
            if tree_node_ref_map.contains_key(&tree_node_parent_id) {
                let current_ref = tree_node_ref_map.get(tree_node.node_id()).unwrap();
                let parent_ref = tree_node_ref_map.get(&tree_node_parent_id).unwrap();
                *current_ref.ref_parent.write().unwrap() = Some(parent_ref.ref_node.unwrap());
                parent_ref.ref_children.write().unwrap().push(current_ref);
            }
        }
    }

    fn build_tree_root_ref_vec<'a, T>(
        tree_node_ref_map: &'a HashMap<String, TreeNodeRef<'a, T>>,
    ) -> Vec<&'a TreeNodeRef<'a, T>>
    where
        T: TreeNode<T> + Clone + Debug + Send + Sync,
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
