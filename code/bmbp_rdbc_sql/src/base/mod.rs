mod sql_value;
mod function;
mod sql_trait;
mod sql_struct;
mod sql_impl;

pub use sql_value::*;
pub use function::*;
pub use sql_trait::*;
pub use sql_struct::*;
pub use sql_impl::*;


#[cfg(test)]
mod test {
    use crate::{BmbpRdbcSQLTable, RdbcSQLTable};

    #[test]
    fn test_table() {
        let table = BmbpRdbcSQLTable::new("table");
        assert_eq!(table.to_table(), "table")
    }
}