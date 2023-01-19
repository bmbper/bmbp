use std::sync::RwLock;

use once_cell::sync::Lazy;

/// BMBP_CACHE 平台接口缓存
pub static DATA_BASE_TYPE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("".to_string()));
