use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub trait QueryPageParam {
    fn get_page_no(&self) -> usize;
    fn get_page_size(&self) -> usize;
}
pub trait BaseOrmVoPo {
    fn get_base_vo(&self) -> &BaseVoPo;
    fn get_mut_base_vo(&mut self) -> &mut BaseVoPo;
    fn set_base_vo(&mut self, vo: BaseVoPo) -> &mut Self;
    fn vo_fields() -> Vec<String>;
    fn orm_fields() -> Vec<String> {
        let mut fields = vec![];
        fields.extend_from_slice(BaseVoPo::vo_fields().as_slice());
        fields.extend_from_slice(Self::vo_fields().as_slice());
        fields
    }
    fn get_r_id(&self) -> &String {
        self.get_base_vo().get_r_id()
    }
    fn get_r_level(&self) -> &String {
        self.get_base_vo().get_r_level()
    }
    fn get_r_flag(&self) -> &String {
        self.get_base_vo().get_r_flag()
    }
    fn get_r_create_time(&self) -> &String {
        self.get_base_vo().get_r_create_time()
    }
    fn get_r_create_user(&self) -> &String {
        self.get_base_vo().get_r_create_user()
    }
    fn get_r_update_time(&self) -> &String {
        self.get_base_vo().get_r_update_time()
    }
    fn get_r_update_user(&self) -> &String {
        self.get_base_vo().get_r_update_user()
    }
    fn get_r_owner_org(&self) -> &String {
        self.get_base_vo().get_r_owner_org()
    }
    fn get_r_owner_user(&self) -> &String {
        self.get_base_vo().get_r_owner_user()
    }
    fn get_r_sign(&self) -> &String {
        self.get_base_vo().get_r_sign()
    }
    fn get_mut_r_id(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_sign()
    }
    fn get_mut_r_level(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_level()
    }
    fn get_mut_r_flag(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_flag()
    }
    fn get_mut_r_create_time(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_create_time()
    }
    fn get_mut_r_create_user(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_create_user()
    }
    fn get_mut_r_update_time(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_update_time()
    }
    fn get_mut_r_update_user(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_update_user()
    }
    fn get_mut_r_owner_org(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_owner_org()
    }
    fn get_mut_r_owner_user(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_owner_user()
    }
    fn get_mut_r_sign(&mut self) -> &mut String {
        self.get_mut_base_vo().get_mut_r_sign()
    }
    fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.get_mut_base_vo().set_r_id(r_id);
        self
    }
    fn set_r_level(&mut self, r_level: String) -> &mut Self {
        self.get_mut_base_vo().set_r_level(r_level);
        self
    }
    fn set_r_flag(&mut self, r_flag: String) -> &mut Self {
        self.get_mut_base_vo().set_r_flag(r_flag);
        self
    }
    fn set_r_create_time(&mut self, r_create_time: String) -> &mut Self {
        self.get_mut_base_vo().set_r_create_time(r_create_time);
        self
    }
    fn set_r_create_user(&mut self, r_create_user: String) -> &mut Self {
        self.get_mut_base_vo().set_r_create_user(r_create_user);
        self
    }
    fn set_r_update_time(&mut self, r_update_time: String) -> &mut Self {
        self.get_mut_base_vo().set_r_update_time(r_update_time);
        self
    }
    fn set_r_update_user(&mut self, r_update_user: String) -> &mut Self {
        self.get_mut_base_vo().set_r_update_user(r_update_user);
        self
    }
    fn set_r_owner_org(&mut self, r_owner_org: String) -> &mut Self {
        self.get_mut_base_vo().set_r_owner_org(r_owner_org);
        self
    }
    fn set_r_owner_user(&mut self, r_owner_user: String) -> &mut Self {
        self.get_mut_base_vo().set_r_owner_user(r_owner_user);
        self
    }
    fn set_r_sign(&mut self, sign: String) -> &mut Self {
        self.get_mut_base_vo().set_r_sign(sign);
        self
    }
}

pub trait TreeNode<T> {
    fn node_id(&self) -> &String;
    fn node_parent_id(&self) -> &String;
    fn node_title(&self) -> &String;
    fn node_data_id(&self) -> &String;
    fn node_path(&self) -> &String;
    fn children(&self) -> &[T];
    fn set_children(&mut self, children: Vec<T>) -> &mut Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpApiQueryParam<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    page_no: usize,
    page_size: usize,
    #[serde(flatten)]
    page_param: Option<T>,
}

impl<T> Default for BmbpApiQueryParam<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    fn default() -> Self {
        Self {
            page_no: 1,
            page_size: 20,
            page_param: None,
        }
    }
}

impl<T> BmbpApiQueryParam<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn new() -> Self {
        BmbpApiQueryParam::default()
    }
    pub fn get_page_param(&self) -> Option<&T> {
        self.page_param.as_ref()
    }
    pub fn set_page_no(&mut self, page_no: usize) {
        self.page_no = page_no;
    }
    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
    }
    pub fn set_page_param(&mut self, param: T) {
        self.page_param = Some(param);
    }
}

impl<T> QueryPageParam for BmbpApiQueryParam<T>
where
    T: Default + Clone + Serialize + Send + Sync,
{
    fn get_page_no(&self) -> usize {
        self.page_no.clone()
    }
    fn get_page_size(&self) -> usize {
        self.page_size.clone()
    }
}

pub type PageReqVo<T> = BmbpApiQueryParam<T>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    page_no: usize,
    page_size: usize,
    total: usize,
    data: Vec<T>,
}

impl<T> PageApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn new() -> Self {
        PageApiResponseVo::default()
    }

    pub fn set_page_no(&mut self, page_no: usize) {
        self.page_no = page_no;
    }
    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
    }
    pub fn set_total(&mut self, total: usize) {
        self.total = total;
    }
    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
    }

    pub fn page_no(&self) -> usize {
        self.page_no.clone()
    }
    pub fn page_size(&self) -> usize {
        self.page_size.clone()
    }
    pub fn total(&self) -> usize {
        self.total.clone()
    }

    pub fn data(&self) -> &[T] {
        self.data.as_slice()
    }
}

pub type PageInner<T> = PageApiResponseVo<T>;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i8)]
pub enum RespStatus {
    SUCCESS = 0i8,
    ERROR = -1i8,
}

impl Default for RespStatus {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl RespStatus {
    pub fn value(&self) -> isize {
        match self {
            RespStatus::SUCCESS => 0,
            RespStatus::ERROR => -1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    code: RespStatus,
    msg: String,
    data: Option<T>,
}

impl<T> Default for BmbpApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    fn default() -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: None,
        }
    }
}

impl<T> BmbpApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn ok() -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: None,
        }
    }

    pub fn ok_msg(msg: String) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: msg,
            data: None,
        }
    }
    pub fn ok_msg_data(msg: String, data: T) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: msg,
            data: Some(data),
        }
    }
    pub fn ok_data(data: T) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: Some(data),
        }
    }

    pub fn ok_option(data: Option<T>) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data,
        }
    }

    pub fn fail() -> Self {
        BmbpApiResponseVo {
            code: RespStatus::ERROR,
            msg: "请求访问失败".to_string(),
            data: None,
        }
    }
    pub fn fail_msg(msg: String) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::ERROR,
            msg: msg,
            data: None,
        }
    }
    pub fn fail_msg_data(msg: String, data: T) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::ERROR,
            msg: msg,
            data: Some(data),
        }
    }
    pub fn fail_data(data: T) -> Self {
        BmbpApiResponseVo {
            code: RespStatus::ERROR,
            msg: "请求访问失败".to_string(),
            data: Some(data),
        }
    }
}

impl<T> BmbpApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn code(&self) -> isize {
        self.code.value()
    }
    pub fn msg(&self) -> String {
        self.msg.clone()
    }
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

pub type RespVo<T> = BmbpApiResponseVo<T>;

impl<T> IntoResponse for BmbpApiResponseVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    fn into_response(self) -> axum::response::Response {
        match serde_json::to_string(&self) {
            Ok(v) => v.into_response(),
            Err(err) => err.to_string().into_response(),
        }
    }
}

pub const ROOT_TREE_NODE: &str = "0";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BaseVoPo {
    ///  记录ID
    r_id: String,
    // 记录级別
    r_level: String,
    // 记录标记
    r_flag: String,
    // 创建时间
    r_create_time: String,
    // 创建用户
    r_create_user: String,
    // 更新时间
    r_update_time: String,
    // 更新人
    r_update_user: String,
    // 所属单位
    r_owner_org: String,
    // 所属人
    r_owner_user: String,
    // 记录签名
    r_sign: String,
}

impl BaseVoPo {
    pub fn new() -> BaseVoPo {
        BaseVoPo::default()
    }
    pub fn vo_fields() -> Vec<String> {
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
        ]
    }
    pub fn get_r_id(&self) -> &String {
        &self.r_id
    }
    pub fn get_r_level(&self) -> &String {
        &self.r_level
    }
    pub fn get_r_flag(&self) -> &String {
        &self.r_flag
    }
    pub fn get_r_create_time(&self) -> &String {
        &self.r_create_time
    }
    pub fn get_r_create_user(&self) -> &String {
        &self.r_create_user
    }
    pub fn get_r_update_time(&self) -> &String {
        &self.r_update_time
    }
    pub fn get_r_update_user(&self) -> &String {
        &self.r_update_user
    }
    pub fn get_r_owner_org(&self) -> &String {
        &self.r_owner_org
    }
    pub fn get_r_owner_user(&self) -> &String {
        &self.r_owner_user
    }
    pub fn get_r_sign(&self) -> &String {
        &self.r_sign
    }
    pub fn get_mut_r_id(&mut self) -> &mut String {
        &mut self.r_id
    }
    pub fn get_mut_r_level(&mut self) -> &mut String {
        &mut self.r_level
    }
    pub fn get_mut_r_flag(&mut self) -> &mut String {
        &mut self.r_flag
    }
    pub fn get_mut_r_create_time(&mut self) -> &mut String {
        &mut self.r_create_time
    }
    pub fn get_mut_r_create_user(&mut self) -> &mut String {
        &mut self.r_create_user
    }
    pub fn get_mut_r_update_time(&mut self) -> &mut String {
        &mut self.r_update_time
    }
    pub fn get_mut_r_update_user(&mut self) -> &mut String {
        &mut self.r_update_user
    }
    pub fn get_mut_r_owner_org(&mut self) -> &mut String {
        &mut self.r_owner_org
    }
    pub fn get_mut_r_owner_user(&mut self) -> &mut String {
        &mut self.r_owner_user
    }
    pub fn get_mut_r_sign(&mut self) -> &mut String {
        &mut self.r_sign
    }
    pub fn set_r_id(&mut self, r_id: String) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub fn set_r_level(&mut self, r_level: String) -> &mut Self {
        self.r_level = r_level;
        self
    }
    pub fn set_r_flag(&mut self, r_flag: String) -> &mut Self {
        self.r_flag = r_flag;
        self
    }
    pub fn set_r_create_time(&mut self, r_create_time: String) -> &mut Self {
        self.r_create_time = r_create_time;
        self
    }
    pub fn set_r_create_user(&mut self, r_create_user: String) -> &mut Self {
        self.r_create_user = r_create_user;
        self
    }
    pub fn set_r_update_time(&mut self, r_update_time: String) -> &mut Self {
        self.r_update_time = r_update_time;
        self
    }
    pub fn set_r_update_user(&mut self, r_update_user: String) -> &mut Self {
        self.r_update_user = r_update_user;
        self
    }
    pub fn set_r_owner_org(&mut self, r_owner_org: String) -> &mut Self {
        self.r_owner_org = r_owner_org;
        self
    }
    pub fn set_r_owner_user(&mut self, r_owner_user: String) -> &mut Self {
        self.r_owner_user = r_owner_user;
        self
    }
    pub fn set_r_sign(&mut self, sign: String) -> &mut Self {
        self.r_sign = sign;
        self
    }
}
