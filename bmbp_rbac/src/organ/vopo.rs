use serde::{Deserialize, Serialize};

use bmbp_types::vo::BaseOrmVoPo;
use bmbp_types::{BaseVoPo, PageReqVo, TreeNode};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct QueryParam {
    r_id: String,
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_type: String,
}

impl QueryParam {
    pub fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub fn set_organ_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    pub fn set_parent_organ_id(&mut self, parent_organ_id: String) -> &mut Self {
        self.parent_organ_id = parent_organ_id;
        self
    }
    pub fn set_organ_title(&mut self, organ_title: String) -> &mut Self {
        self.organ_title = organ_title;
        self
    }
    pub fn set_organ_path(&mut self, organ_path: String) -> &mut Self {
        self.organ_path = organ_path;
        self
    }
    pub fn set_organ_type(&mut self, organ_type: String) -> &mut Self {
        self.organ_type = organ_type;
        self
    }
}

// 分页查询参数
pub type PageQueryParam = PageReqVo<QueryParam>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BmbpOrganType {
    Unit,
    Units,
    Dept,
    Person,
}

impl Default for BmbpOrganType {
    fn default() -> Self {
        BmbpOrganType::Unit
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganVo {
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_data_id: String,
    organ_type: BmbpOrganType,
    children: Vec<BmbpOrganVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

impl BaseOrmVoPo for BmbpOrganVo {
    fn get_base_vo(&self) -> &BaseVoPo {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BaseVoPo) {
        self.base = vo;
    }

    fn vo_fields() -> Vec<String> {
        vec![
            "organ_id".to_string(),
            "parent_organ_id".to_string(),
            "organ_title".to_string(),
            "organ_path".to_string(),
            "organ_type".to_string(),
            "organ_data_id".to_string(),
        ]
    }
}

impl BmbpOrganVo {
    pub fn new() -> Self {
        BmbpOrganVo::default()
    }
}

impl TreeNode<BmbpOrganVo> for BmbpOrganVo {
    fn node_id(&self) -> &String {
        &self.organ_id
    }
    fn node_parent_id(&self) -> &String {
        &self.parent_organ_id
    }
    fn node_title(&self) -> &String {
        &self.organ_title
    }
    fn node_data_id(&self) -> &String {
        &self.organ_data_id
    }
    fn node_path(&self) -> &String {
        &self.organ_path
    }
    fn children(&self) -> &[BmbpOrganVo] {
        self.children.as_slice()
    }
    fn set_children(&mut self, children: Vec<BmbpOrganVo>) {
        self.children = children
    }
}

#[cfg(test)]
mod tests {
    use bmbp_types::BaseVoPo;
    use bmbp_util::TreeBuilder;

    use crate::organ::vopo::BmbpOrganType;

    use super::BmbpOrganVo;

    #[test]
    fn test_tree() {
        let mut organ_vec: Vec<BmbpOrganVo> = vec![];
        let root = BmbpOrganVo {
            organ_id: "1100".to_string(),
            parent_organ_id: "11".to_string(),
            organ_title: "1100".to_string(),
            organ_path: "11/1100".to_string(),
            organ_data_id: "11".to_string(),
            organ_type: BmbpOrganType::Units,
            children: vec![],
            base: BaseVoPo::default(),
        };
        organ_vec.push(root);

        let root2 = BmbpOrganVo {
            organ_id: "1200".to_string(),
            parent_organ_id: "11".to_string(),
            organ_title: "1200".to_string(),
            organ_path: "11/1200".to_string(),
            organ_data_id: "11".to_string(),
            organ_type: BmbpOrganType::Units,
            children: vec![],
            base: BaseVoPo::default(),
        };
        //  organ_vec.push(root2);

        let child1 = BmbpOrganVo {
            organ_id: "110011".to_string(),
            parent_organ_id: "1100".to_string(),
            organ_title: "110011".to_string(),
            organ_path: "11/1100/110011".to_string(),
            organ_data_id: "1100".to_string(),
            organ_type: BmbpOrganType::Unit,
            children: vec![],
            base: BaseVoPo::default(),
        };

        // organ_vec.push(child1);

        let organ_tree = TreeBuilder::build::<BmbpOrganVo>(organ_vec);
        println!("{}", serde_json::to_string_pretty(&organ_tree).unwrap());
    }
}
