use crate::RdbcTableVo;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcSchemaVo {
    name: String,
    comment: Option<String>,
    owner: Option<String>,
    charset: Option<String>,
    tables: Vec<RdbcTableVo>,
}
