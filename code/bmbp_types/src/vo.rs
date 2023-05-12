use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub trait TreeNode<T> {
    fn get_tree_id(&self) -> &String;
    fn get_tree_parent_id(&self) -> &String;
    fn get_tree_data_id(&self) -> &String;
    fn get_tree_title(&self) -> &String;
    fn get_tree_id_path(&self) -> &String;
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageRespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    page_no: usize,
    page_size: usize,
    total: usize,
    data: Option<Vec<T>>,
}

impl<T> PageRespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn new() -> Self {
        PageRespVo::default()
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
        self.data = Some(data);
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

    pub fn data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
}

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
pub struct RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    code: RespStatus,
    msg: String,
    data: Option<T>,
}

impl<T> Default for RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    fn default() -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: None,
        }
    }
}

impl<T> RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn ok() -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: None,
        }
    }

    pub fn ok_msg(msg: String) -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: msg,
            data: None,
        }
    }
    pub fn ok_msg_data(msg: String, data: T) -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: msg,
            data: Some(data),
        }
    }
    pub fn ok_data(data: T) -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data: Some(data),
        }
    }

    pub fn ok_option(data: Option<T>) -> Self {
        RespVo {
            code: RespStatus::SUCCESS,
            msg: "请求访问成功".to_string(),
            data,
        }
    }

    pub fn fail() -> Self {
        RespVo {
            code: RespStatus::ERROR,
            msg: "请求访问失败".to_string(),
            data: None,
        }
    }
    pub fn fail_msg(msg: String) -> Self {
        RespVo {
            code: RespStatus::ERROR,
            msg: msg,
            data: None,
        }
    }
    pub fn fail_msg_data(msg: String, data: T) -> Self {
        RespVo {
            code: RespStatus::ERROR,
            msg: msg,
            data: Some(data),
        }
    }
    pub fn fail_data(data: T) -> Self {
        RespVo {
            code: RespStatus::ERROR,
            msg: "请求访问失败".to_string(),
            data: Some(data),
        }
    }
}

impl<T> RespVo<T>
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

impl<T> IntoResponse for RespVo<T>
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
