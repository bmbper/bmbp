/// BmbpTree 树型结构的标准接口
pub trait BmbpTree<T> {
    fn get_tree_id(&self) -> &String;
    fn get_tree_id_path(&self) -> &String;
    fn get_tree_parent_id(&self) -> &String;
    fn get_tree_data_id(&self) -> &String;
    fn get_tree_title(&self) -> &String;
    fn get_tree_title_path(&self) -> &String;
    fn get_tree_children(&self) -> &[T];
    fn get_mut_tree_children(&mut self) -> &mut Vec<T>;
    fn set_tree_id(&mut self, tree_id: String) -> &mut Self;
    fn set_tree_parent_id(&mut self, tree_id: String) -> &mut Self;
    fn set_tree_data_id(&mut self, tree_id: String) -> &mut Self;
    fn set_tree_title(&mut self, tree_title: String) -> &mut Self;
    fn set_tree_id_path(&mut self, tree_id_path: String) -> &mut Self;
    fn set_tree_title_path(&mut self, tree_title_path: String) -> &mut Self;
    fn set_tree_children(&mut self, children: Vec<T>) -> &mut Self;
}
