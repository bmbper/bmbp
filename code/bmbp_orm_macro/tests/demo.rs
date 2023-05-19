use serde;

use bmbp_types::BmbpTree;

#[model]
pub struct Demo {
    organ_id: String,
    organ_id_path: String,
    organ_parent_id: String,
    organ_data_id: String,
    organ_title: String,
    organ_title_path: String,
    #[skip]
    organ_children: Vec<Demo>,
}

impl BmbpTree<Demo> for Demo {
    fn get_tree_id(&self) -> &String {
        &self.organ_id
    }
    fn get_tree_parent_id(&self) -> &String {
        &self.organ_parent_id
    }
    fn get_tree_data_id(&self) -> &String {
        &self.organ_data_id
    }
    fn get_tree_title(&self) -> &String {
        &self.organ_title
    }
    fn get_tree_id_path(&self) -> &String {
        &self.organ_id_path
    }
    fn get_tree_title_path(&self) -> &String {
        &self.organ_title_path
    }
    fn get_tree_children(&self) -> &[Demo] {
        self.organ_children.as_slice()
    }
    fn get_mut_tree_children(&mut self) -> &mut Vec<Demo> {
        &mut self.organ_children
    }

    fn set_tree_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    fn set_tree_parent_id(&mut self, organ_parent_id: String) -> &mut Self {
        self.organ_parent_id = organ_parent_id;
        self
    }
    fn set_tree_data_id(&mut self, organ_data_id: String) -> &mut Self {
        self.organ_data_id = organ_data_id;
        self
    }
    fn set_tree_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = organ_title;
        self
    }
    fn set_tree_id_path(&mut self, organ_id_path: String) -> &mut Self {
        self.organ_id_path = organ_id_path;
        self
    }
    fn set_tree_title_path(&mut self, organ_title_path: String) -> &mut Self {
        self.organ_title_path = organ_title_path;
        self
    }
    fn set_tree_children(&mut self, organ_children: Vec<Demo>) -> &mut Self {
        self.organ_children = organ_children;
        self
    }
}
