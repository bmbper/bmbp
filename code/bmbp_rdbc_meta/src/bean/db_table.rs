use crate::{
    RdbcColumnVo, RdbcTableCheckVo, RdbcTableForeignKeyVo, RdbcTableIndexVo, RdbcTablePrimaryKeyVo,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcTableVo {
    name: String,
    comment: Option<String>,
    charset: Option<String>,
    schema: Option<String>,
    columns: Vec<RdbcColumnVo>,
    primary_key: Option<RdbcTablePrimaryKeyVo>,
    foreign_key: Option<Vec<RdbcTableForeignKeyVo>>,
    check: Option<Vec<RdbcTableCheckVo>>,
    index: Option<Vec<RdbcTableIndexVo>>,
}
