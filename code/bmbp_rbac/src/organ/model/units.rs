use bmbp_types::vo::BaseOrmModel;
use bmbp_types::BmbpBaseModel;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganUnitsVo {
    organ_id: String,
    units_desc: String,
    #[serde(flatten)]
    base: BmbpBaseModel,
}

#[allow(dead_code)]
impl BmbpOrganUnitsVo {
    pub fn new() -> Self {
        BmbpOrganUnitsVo::default()
    }
}

#[allow(dead_code)]
impl BmbpOrganUnitsVo {
    pub fn organ_id(&self) -> &String {
        &self.organ_id
    }
    pub fn organ_desc(&self) -> &String {
        &self.units_desc
    }
    pub fn set_organ_id(&mut self, organ_id: String) -> &mut Self {
        self.organ_id = organ_id;
        self
    }
    pub fn set_organ_desc(&mut self, units_desc: String) -> &mut Self {
        self.units_desc = units_desc;
        self
    }
}

#[allow(dead_code)]
impl BaseOrmModel for BmbpOrganUnitsVo {
    fn get_base_vo(&self) -> &BmbpBaseModel {
        &self.base
    }

    fn get_mut_base_vo(&mut self) -> &mut BmbpBaseModel {
        &mut self.base
    }

    fn set_base_vo(&mut self, vo: BmbpBaseModel) -> &mut Self {
        self.base = vo;
        self
    }
    fn vo_fields() -> Vec<String> {
        vec!["organ_id".to_string(), "units_desc".to_string()]
    }
}
