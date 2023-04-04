#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Demo1 {
    r_id: String,
    r_level: String,
    r_flag: String,
    r_create_time: String,
    r_create_user: String,
    r_update_time: String,
    r_update_user: String,
    r_owner_org: String,
    r_owner_user: String,
    r_sign: String,
    name: String,
    id: String,
    children: Vec<String>,
}

impl Demo1 {
    pub fn set_r_id(&mut self, v: String) -> &mut Self {
        self.r_id = v;
        self
    }
    pub fn get_r_id(&mut self) -> &String {
        &self.r_id
    }
    pub fn get_mut_r_id(&mut self) -> &mut String {
        &mut self.r_id
    }
    pub fn set_r_level(&mut self, v: String) -> &mut Self {
        self.r_level = v;
        self
    }
    pub fn get_r_level(&mut self) -> &String {
        &self.r_level
    }
    pub fn get_mut_r_level(&mut self) -> &mut String {
        &mut self.r_level
    }
    pub fn set_r_flag(&mut self, v: String) -> &mut Self {
        self.r_flag = v;
        self
    }
    pub fn get_r_flag(&mut self) -> &String {
        &self.r_flag
    }
    pub fn get_mut_r_flag(&mut self) -> &mut String {
        &mut self.r_flag
    }
    pub fn set_r_create_time(&mut self, v: String) -> &mut Self {
        self.r_create_time = v;
        self
    }
    pub fn get_r_create_time(&mut self) -> &String {
        &self.r_create_time
    }
    pub fn get_mut_r_create_time(&mut self) -> &mut String {
        &mut self.r_create_time
    }
    pub fn set_r_create_user(&mut self, v: String) -> &mut Self {
        self.r_create_user = v;
        self
    }
    pub fn get_r_create_user(&mut self) -> &String {
        &self.r_create_user
    }
    pub fn get_mut_r_create_user(&mut self) -> &mut String {
        &mut self.r_create_user
    }
    pub fn set_r_update_time(&mut self, v: String) -> &mut Self {
        self.r_update_time = v;
        self
    }
    pub fn get_r_update_time(&mut self) -> &String {
        &self.r_update_time
    }
    pub fn get_mut_r_update_time(&mut self) -> &mut String {
        &mut self.r_update_time
    }
    pub fn set_r_update_user(&mut self, v: String) -> &mut Self {
        self.r_update_user = v;
        self
    }
    pub fn get_r_update_user(&mut self) -> &String {
        &self.r_update_user
    }
    pub fn get_mut_r_update_user(&mut self) -> &mut String {
        &mut self.r_update_user
    }
    pub fn set_r_owner_org(&mut self, v: String) -> &mut Self {
        self.r_owner_org = v;
        self
    }
    pub fn get_r_owner_org(&mut self) -> &String {
        &self.r_owner_org
    }
    pub fn get_mut_r_owner_org(&mut self) -> &mut String {
        &mut self.r_owner_org
    }
    pub fn set_r_owner_user(&mut self, v: String) -> &mut Self {
        self.r_owner_user = v;
        self
    }
    pub fn get_r_owner_user(&mut self) -> &String {
        &self.r_owner_user
    }
    pub fn get_mut_r_owner_user(&mut self) -> &mut String {
        &mut self.r_owner_user
    }
    pub fn set_r_sign(&mut self, v: String) -> &mut Self {
        self.r_sign = v;
        self
    }
    pub fn get_r_sign(&mut self) -> &String {
        &self.r_sign
    }
    pub fn get_mut_r_sign(&mut self) -> &mut String {
        &mut self.r_sign
    }
    pub fn get_orm_table() -> String {
        "DEMO1".to_string()
    }
    pub fn set_name(&mut self, v: String) -> &mut Self {
        self.name = v;
        self
    }
    pub fn get_name(&mut self) -> &String {
        &self.name
    }
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn set_id(&mut self, v: String) -> &mut Self {
        self.id = v;
        self
    }
    pub fn get_id(&mut self) -> &String {
        &self.id
    }
    pub fn get_mut_id(&mut self) -> &mut String {
        &mut self.id
    }
    pub fn set_children(&mut self, v: Vec<String>) -> &mut Self {
        self.children = v;
        self
    }
    pub fn get_children(&mut self) -> &Vec<String> {
        &self.children
    }
    pub fn get_mut_children(&mut self) -> &mut Vec<String> {
        &mut self.children
    }
    pub fn get_orm_fields(&self) -> Vec<String> {
        vec![
            "r_id".to_string(),
            "r_level".to_string(),
            "r_flag".to_string(),
            "r_create_time".to_string(),
            "r_create_user".to_string(),
            "r_update_time".to_string(),
            "r_update_user".to_string(),
            "r_owner_org".to_string(),
            "r_owner_user".to_string(),
            "r_sign".to_string(),
            "name".to_string(),
            "id".to_string(),
        ]
    }
}
