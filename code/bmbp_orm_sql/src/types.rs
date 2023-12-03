pub trait SqlBuilder {
    fn build(&self) -> String;
}

#[derive(Clone)]
pub enum SelectBuilder {
    String(String),
    // 字段名 | 字段名 AS 别名 | 表名.字段名 | 表名.字段名 AS 别名 | SQL语句 | SQL语句 AS 别名 | 批量
    Dynamic(DynamicSelectBuilder),// 动态字段
}

#[derive(Clone)]
pub struct DynamicSelectBuilder {
    pub(crate) table_alias_: Option<String>,
    // 表别名
    pub(crate) field_: Box<DynamicSelectBuilderType>,
    // 动态字段
    pub(crate) alias_: Option<String>, // 字段别名
}


#[derive(Clone)]
pub enum DynamicSelectBuilderType {
    // 字段名 | 字段名 AS 别名 | 表名.字段名 | 表名.字段名 AS 别名 | SQL语句 | SQL语句 AS 别名 | 批量
    String(String),
    // 动态字段
    Dynamic(Box<DynamicSelectBuilder>),
    // 临时表 QueryBuilder
    TempTable(QueryBuilder),
}


#[derive(Clone)]
pub enum TableBuilder {
    String(String),
    Dynamic(DynamicTableBuilder),
}


#[derive(Clone)]
pub struct DynamicTableBuilder {
    // 数据库实例
    pub(crate) schema: Option<String>,
    // 表名
    pub(crate) table: Box<DynamicTableBuilderType>,
    // 表别名
    pub(crate) alias: Option<String>,
}


#[derive(Clone)]
pub enum DynamicTableBuilderType {
    // 表名  | 临时表
    String(String),
    // 动态表名
    Dynamic(Box<DynamicTableBuilder>),
    // 临时表 表构建器
    TempTable(TableBuilder),
}


#[derive(Clone)]
pub struct QueryBuilder {
    pub(crate) select_: Vec<SelectBuilder>,
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) join_: Vec<JoinQueryBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
    pub(crate) group_: Option<Vec<SelectBuilder>>,
    pub(crate) order_: Option<Vec<OrderBuilder>>,
    pub(crate) limit_: Option<u64>,
    pub(crate) offset_: Option<u64>,
}


#[derive(Clone)]
pub struct JoinQueryBuilder {
    pub(crate) table: TableBuilder,
    pub(crate) typ: JoinTableType,
    pub(crate) filter: Option<QueryFilterBuilder>,
}


#[derive(Clone)]
pub enum JoinTableType {
    Inner,
    Left,
    Right,
    Full,
}


#[derive(Clone)]
pub struct QueryFilterBuilder {
    pub(crate) typ: QueryFilterType,
    pub(crate) filters: Vec<QueryFilterItemBuilder>,
}

#[derive(Clone)]
pub enum QueryFilterType {
    And,
    Or,
}


#[derive(Clone)]
pub enum QueryFilterItemBuilder {
    String(String),
    Simple(QuerySimpleFilterItemBuilder),
    Nested(QueryFilterBuilder),
}

impl QueryFilterItemBuilder {
    pub fn filter(filter: &str) -> QueryFilterItemBuilder {
        QueryFilterItemBuilder::String(filter.to_string())
    }
}

#[derive(Clone)]
pub struct QuerySimpleFilterItemBuilder {
    pub(crate) filter_typ_: QueryFilterType,
    pub(crate) field_: FilterFieldBuilder,
    pub(crate) value_: String,
    pub(crate) op_: FilterOperatorType,
}


#[derive(Clone)]
pub enum FilterFieldBuilder {
    String(String),
    Dynamic(DynamicFilterFieldBuilder),
}


#[derive(Clone)]
pub enum DynamicFilterFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(QueryBuilder),
}


#[derive(Clone)]
pub enum FilterOperatorType {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}


#[derive(Clone)]
pub struct OrderBuilder {
    pub(crate) field: String,
    pub(crate) typ: OrderType,
}


#[derive(Clone)]
pub enum OrderFieldBuilder {
    String(String),
    Dynamic(DynamicOrderFieldBuilder),
}


#[derive(Clone)]
pub enum DynamicOrderFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(FilterFieldBuilder),
}


#[derive(Clone)]
pub enum OrderType {
    Asc,
    Desc,
}


#[derive(Clone)]
pub struct UpdateBuilder {
    pub(crate) set_: Vec<UpdateSetFieldBuilder>,
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) join_: Vec<JoinQueryBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
}


#[derive(Clone)]
pub struct UpdateSetFieldBuilder {
    pub(crate) field_: FilterFieldBuilder,
    pub(crate) value_: String,
}


#[derive(Clone)]
pub struct DeleteBuilder {
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
}


#[derive(Clone)]
pub struct InsertBuilder {
    pub(crate) into_: Option<TableBuilder>,
    pub(crate) fields_: Option<Vec<String>>,
    pub(crate) values_: Option<Vec<String>>,
    pub(crate) query_: Option<QueryBuilder>,
}