use crate::{RdbcSQLSelect, RdbcSQLTable, RdbcSQLValue};

impl<T> RdbcSQLTable for T where T: ToString {
    fn to_table(&self) -> String {
        self.to_string()
    }
}

impl<T> RdbcSQLSelect for T where T: ToString {
    fn to_select(&self) -> String {
        self.to_string()
    }
}

