use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableIndexVo {
    name: String,
    comment: Option<String>,
    columns: Vec<String>,
    unique: Option<bool>,
}
