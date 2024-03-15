use bmbp_rdbc_sql::{Delete, Insert, Query, RdbcSQL, RdbcValue, Update};

pub struct PgSQL;

impl RdbcSQL for PgSQL {
    fn to_query(query: &Query) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_insert(query: &Insert) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_update(query: &Update) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_delete(query: &Delete) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

pub struct MysqlSQL;

impl RdbcSQL for MysqlSQL {
    fn to_query(query: &Query) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_insert(insert: &Insert) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_update(update: &Update) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_delete(delete: &Delete) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}

pub struct SqliteSQL;

impl RdbcSQL for SqliteSQL {
    fn to_query(query: &Query) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_insert(query: &Insert) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_update(query: &Update) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn to_delete(query: &Delete) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
}
