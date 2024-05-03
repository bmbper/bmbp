use async_trait::async_trait;
use salvo::{Depot, Request, Response, Writer, writing::Json};
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
    row_data: Option<Vec<T>>,
}

impl<T> PageVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    pub fn new() -> Self {
        PageVo::default()
    }

    pub fn new_page(
        page_no: usize,
        page_size: usize,
        row_total: usize,
        data: Option<Vec<T>>,
    ) -> Self {
        PageVo {
            page_no: Some(page_no),
            page_size: Some(page_size),
            row_total: Some(row_total),
            row_data: data,
        }
    }

    pub fn ok_data(data: Vec<T>) -> Self {
        PageVo {
            page_no: Some(0),
            page_size: Some(0),
            row_total: Some(data.len()),
            row_data: Some(data),
        }
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
        self.row_data = Some(data);
        self
    }
    pub fn set_op_data(&mut self, data: Option<Vec<T>>) -> &mut Self {
        self.row_data = data;
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
        self.row_data.as_ref()
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i16)]
pub enum RespCode {
    SUCCESS = 0i16,
    ERROR = -1i16,
    NotFound = 404i16,
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
    pub fn ok_find_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("查询成功".to_string()),
            data: Some(data),
        }
    }
    pub fn ok_save_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("保存成功".to_string()),
            data: Some(data),
        }
    }
    pub fn ok_remove_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("删除成功".to_string()),
            data: Some(data),
        }
    }
    pub fn ok_enable_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("启用成功".to_string()),
            data: Some(data),
        }
    }

    pub fn ok_disable_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("停用成功".to_string()),
            data: Some(data),
        }
    }
    pub fn ok_publish_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("发布成功".to_string()),
            data: Some(data),
        }
    }
    pub fn ok_cancel_data(data: T) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("取消成功".to_string()),
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
    pub fn ok_save_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("保存成功".to_string()),
            data,
        }
    }
    pub fn ok_find_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("查询成功".to_string()),
            data,
        }
    }
    pub fn ok_update_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("更新成功".to_string()),
            data,
        }
    }
    pub fn ok_submit_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("提交成功".to_string()),
            data,
        }
    }
    pub fn ok_remove_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("删除成功".to_string()),
            data,
        }
    }
    pub fn ok_enable_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("启用成功".to_string()),
            data,
        }
    }
    pub fn ok_disable_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("停用成功".to_string()),
            data,
        }
    }
    pub fn ok_publish_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("发布成功".to_string()),
            data,
        }
    }
    pub fn ok_cancel_option(data: Option<T>) -> Self {
        RespVo {
            code: Some(RespCode::SUCCESS),
            msg: Some("取消成功".to_string()),
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

/// 定义包含异常的返回类型
pub type BmbpResp<T> = Result<T, BmbpError>;

#[async_trait]
impl<T> Writer for RespVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self))
    }
}

#[async_trait]
impl<T> Writer for PageVo<T>
where
    T: Clone + Default + Serialize + Send + Sync,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self))
    }
}

pub type HttpRespVo<T> = BmbpResp<RespVo<T>>;
pub type HttpRespListVo<T> = BmbpResp<RespVo<Vec<T>>>;
pub type HttpRespPageVo<T> = BmbpResp<RespVo<PageVo<T>>>;
