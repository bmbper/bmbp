use crate::RdbcColumn;

pub enum RdbcFunc {
    CONCAT(RdbcConcatFunc),
}

impl RdbcFunc {
    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcFunc {
        RdbcFunc::CONCAT(RdbcConcatFunc::concat_split(columns, split_))
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcFunc {
        RdbcFunc::CONCAT(RdbcConcatFunc::concat(columns))
    }
}


pub struct RdbcConcatFunc {
    liter_: Option<String>,
    columns_: Vec<RdbcColumn>,
}

impl RdbcConcatFunc {
    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcConcatFunc {
        RdbcConcatFunc {
            liter_: split_,
            columns_: columns,
        }
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcConcatFunc {
        RdbcConcatFunc {
            liter_: None,
            columns_: columns,
        }
    }
}
