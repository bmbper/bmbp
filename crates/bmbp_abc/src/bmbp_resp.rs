use crate::BmbpResp;
use bmbp_bean::{RespVo, VecRespVo};
use salvo::prelude::Json;

pub type VecResp<T> = BmbpResp<Json<VecRespVo<T>>>;
pub type Resp<T> = BmbpResp<Json<RespVo<T>>>;
