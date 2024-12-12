use crate::BmbpResp;
use bmbp_bean::{PageRespVo, RespVo, VecRespVo};

pub type VecResp<T> = BmbpResp<VecRespVo<T>>;
pub type Resp<T> = BmbpResp<RespVo<T>>;
pub type PageResp<T> = BmbpResp<PageRespVo<T>>;
