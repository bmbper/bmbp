use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTablePrimaryKeyVo {
    name: String,
    comment: Option<String>,
    columns: Vec<String>,
}
