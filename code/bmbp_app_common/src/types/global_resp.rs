use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::BmbpError;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    page_no: Option<usize>,
    page_size: Option<usize>,
    row_total: Option<usize>,
    data: Option<Vec<T>>,
}

impl<T> PageVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn new() -> Self {
        PageVo::default()
    }

    pub fn set_page_no(&mut self, page_no: usize) -> &mut Self {
        self.page_no = Some(page_no);
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn set_row_total(&mut self, row_total: usize) -> &mut Self {
        self.row_total = Some(row_total);
        self
    }
    pub fn set_data(&mut self, data: Vec<T>) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn get_page_no(&self) -> Option<&usize> {
        self.page_no.as_ref()
    }

    pub fn get_page_size(&self) -> Option<&usize> {
        self.page_size.as_ref()
    }

    pub fn get_row_total(&self) -> Option<&usize> {
        self.row_total.as_ref()
    }

    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i8)]
pub enum RespCode {
    SUCCESS = 0i8,
    ERROR = -1i8,
}
impl Default for RespCode {
    fn default() -> Self {
        Self::SUCCESS
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    code: Option<RespCode>,
    msg: Option<String>,
    data: Option<T>,
}

impl<T> RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn get_code(&self) -> Option<&RespCode> {
        self.code.as_ref()
    }
    pub fn get_msg(&self) -> Option<&String> {
        self.msg.as_ref()
    }
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    pub fn set_code(&mut self, code: RespCode) -> &mut Self {
        self.code = Some(code);
        self
    }
    pub fn set_msg(&mut self, msg: &str) -> &mut Self {
        self.msg = Some(msg.to_string());
        self
    }
    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }
}

impl<T> RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn ok() -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("请求访问成功".to_string()),
            data: None,
        }
    }

    pub fn ok_msg(msg: &str) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some(msg.to_string()),
            data: None,
        }
    }
    pub fn ok_msg_data(msg: &str, data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some(msg.to_string()),
            data: Some(data),
        }
    }
    pub fn ok_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("请求访问成功".to_string()),
            data: Some(data),
        }
    }

    pub fn ok_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("请求访问成功".to_string()),
            data,
        }
    }
    pub fn ok_msg_option(msg: &str, data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some(msg.to_string()),
            data,
        }
    }

    pub fn fail() -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some("请求访问失败".to_string()),
            data: None,
        }
    }
    pub fn fail_msg(msg: &str) -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some(msg.to_string()),
            data: None,
        }
    }
    pub fn fail_msg_data(msg: &str, data: T) -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some(msg.to_string()),
            data: Some(data),
        }
    }
    pub fn fail_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some("请求访问失败".to_string()),
            data: Some(data),
        }
    }
    pub fn fail_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some("请求访问失败".to_string()),
            data,
        }
    }
    pub fn fail_msg_option(msg: &str, data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::ERROR),
            msg: Some(msg.to_string()),
            data,
        }
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

/// 定义包含异常的返回类型
pub type BmbpResp<T> = Result<T, BmbpError>;