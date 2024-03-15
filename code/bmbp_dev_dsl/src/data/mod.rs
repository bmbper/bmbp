// 数据库模块定义规范
// 本模块定义了数据库模块的规范，包括DDL、DML、DQL
//      DDL包括数据库表结构、数据库表字段、数据库表索引、数据库表约束等
//      DML包括数据库表数据插入、数据库表数据更新、数据库表数据删除等
//      DQL包括数据库表数据查询、数据库表数据统计等
// 本模块采用JSON schema 规范进行描述，仅作定义

mod ddl;
mod dmql;

pub use ddl::*;
pub use dmql::*;
