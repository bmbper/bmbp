use crate::BmbpResp;
use bmbp_bean::{PageRespVo, RespVo, VecRespVo};
use salvo::prelude::Json;

pub type VecResp<T> = BmbpResp<Json<VecRespVo<T>>>;
pub type Resp<T> = BmbpResp<Json<RespVo<T>>>;
pub type PageResp<T> = BmbpResp<Json<PageRespVo<T>>>;
