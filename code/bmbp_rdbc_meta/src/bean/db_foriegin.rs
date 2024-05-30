use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableForeignKeyVo {
    name: String,
    comment: Option<String>,
}
