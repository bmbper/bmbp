use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableCheckVo {
    name: String,
    comment: Option<String>,
    target_table: Option<String>,
    columns: Vec<(String, String)>,
    delete_rule: Option<String>,
    update_rule: Option<String>,
}
