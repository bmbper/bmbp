mod dsl_struct;
mod dsl_func;
mod dsl_trait;
mod dsl_value;

pub use dsl_struct::*;
pub use dsl_func::*;
pub use dsl_trait::*;
pub use dsl_value::*;

pub enum DatabaseType{
    MySQL,
    SQLite,
    Postgres,
    MSSQL,
    Oracle,
    DB2,
    Firebird,
    Informix,
    Sybase,
    Access,
    Ingres,
    Interbase,
    SQLite3,
    Virtuoso,
    H2,
    HSQLDB,
    MemSQL,
    MonetDB,
    Crate,
    Redshift,
    ClickHouse,
    BigQuery,
    Snowflake,
    Presto,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_table() {
    }
}