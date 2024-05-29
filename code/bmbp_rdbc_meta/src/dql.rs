use crate::bean::{RdbcColumnVo, RdbcSchemaVo, RdbcTableVo};
use serde::de::Unexpected::Option;

pub fn find_schemas() -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_matcher(_matcher: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_owner(_owner: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}

pub fn find_schemas_with_table() -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_matcher_with_table(_matcher: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_owner_with_table(_owner: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_with_table_column() -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_matcher_with_table_column(_matcher: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}
pub fn find_schemas_by_owner_with_table_column(_owner: String) -> Option<Vec<RdbcSchemaVo>> {
    None
}

pub fn find_tables() -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schema(_schema: String) -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schemas(_schema: &[String]) -> Option<Vec<RdbcTableVo>> {
    None
}
pub fn find_tables_by_schema_matcher(_schema: String, _table: String) -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schemas_matcher(
    _schema: &[String],
    _table: String,
) -> Option<Vec<RdbcTableVo>> {
    None
}
pub fn find_tables_with_column() -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schema_with_column(_schema: String) -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schemas_with_column(_schema: &[String]) -> Option<Vec<RdbcTableVo>> {
    None
}
pub fn find_tables_by_schema_matcher_with_column(
    _schema: String,
    _table: String,
) -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_tables_by_schemas_matcher_with_column(
    _schema: &[String],
    _table: String,
) -> Option<Vec<RdbcTableVo>> {
    None
}

pub fn find_columns() -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_match_schema(_schema: String) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_match_schemas(_schema: &[String]) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_by_schema(_schema: String) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_by_schemas(_schema: &[String]) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_match_schema_table(
    _schema: String,
    _table: String,
) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_match_schemas_table(
    _schema: &[String],
    _table: String,
) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_by_schema_table(_schema: String, _table: String) -> Option<Vec<RdbcColumnVo>> {
    None
}
pub fn find_columns_by_schemas_table(
    _schema: &[String],
    _table: String,
) -> Option<Vec<RdbcColumnVo>> {
    None
}
