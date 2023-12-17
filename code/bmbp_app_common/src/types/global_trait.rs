/// BmbpTree 树型结构的标准接口
pub trait BmbpTree<T> {
    fn get_tree_code(&self) -> Option<&String>;
    fn get_tree_code_path(&self) -> Option<&String>;
    fn get_tree_parent_code(&self) -> Option<&String>;
    fn get_tree_data_id(&self) -> Option<&String>;
    fn get_tree_title(&self) -> Option<&String>;
    fn get_tree_title_path(&self) -> Option<&String>;
    fn get_tree_children(&self) -> Option<&Vec<T>>;
    fn get_mut_tree_children(&mut self) -> Option<&mut Vec<T>>;

    fn set_tree_code(&mut self, tree_code: String) -> &mut Self;
    fn set_tree_code_path(&mut self, tree_id_path: String) -> &mut Self;
    fn set_tree_parent_code(&mut self, tree_parent_code: String) -> &mut Self;
    fn set_tree_data_id(&mut self, tree_data_id: String) -> &mut Self;
    fn set_tree_title(&mut self, tree_title: String) -> &mut Self;
    fn set_tree_title_path(&mut self, tree_title_path: String) -> &mut Self;
    fn set_tree_children(&mut self, children: Vec<T>) -> &mut Self;
}

/// BmbpTree 树型结构的标准接口
pub trait BmbpBaseModelTrait {
    fn get_base_r_id(&self) -> &String;
    fn get_base_r_flag(&self) -> &String;
    fn get_base_r_level(&self) -> &String;
    fn get_base_r_create_time(&self) -> &String;
    fn get_base_r_create_user(&self) -> &String;
    fn get_base_r_update_time(&self) -> &String;
    fn get_base_r_update_user(&self) -> &String;
    fn get_base_r_owner_org(&self) -> &String;
    fn get_base_r_owner_user(&self) -> &String;
    fn get_base_r_sign(&self) -> &String;

    fn set_base_r_id(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_flag(&mut self, r_flag: String) -> &mut Self;
    fn set_base_r_level(&mut self, r_flag: String) -> &mut Self;
    fn set_base_r_create_time(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_create_user(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_update_time(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_update_user(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_owner_org(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_owner_user(&mut self, r_id: String) -> &mut Self;
    fn set_base_r_sign(&mut self, r_id: String) -> &mut Self;
}

