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
    pub(crate) field_: DynamicSelectBuilderType,
    // 动态字段
    pub(crate) alias_: Option<String>, // 字段别名
}


#[derive(Clone, Default)]
pub enum DynamicSelectBuilderType {
    // 字段名 | 字段名 AS 别名 | 表名.字段名 | 表名.字段名 AS 别名 | SQL语句 | SQL语句 AS 别名 | 批量
    String(String),
    // 动态字段
    Dynamic(DynamicSelectBuilder),
    // 临时表 QueryBuilder
    TempTable(QueryBuilder),
}


#[derive(Clone, Default)]
pub enum TableBuilder {
    String(String),
    Dynamic(DynamicTableBuilder),
}


#[derive(Clone, Default)]
pub struct DynamicTableBuilder {
    // 数据库实例
    pub(crate) schema: Option<String>,
    // 表名
    pub(crate) table: DynamicTableBuilderType,
    // 表别名
    pub(crate) alias: Option<String>,
}


#[derive(Clone, Default)]
pub enum DynamicTableBuilderType {
    // 表名  | 临时表
    String(String),
    // 动态表名
    Dynamic(DynamicTableBuilder),
    // 临时表 表构建器
    TempTable(TableBuilder),
}


#[derive(Clone, Default)]
pub struct QueryBuilder {
    pub(crate) select_: Vec<SelectBuilder>,
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) join_: Vec<JoinQueryBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
    pub(crate) group_: Option<Vec<QueryFilterItemBuilder>>,
    pub(crate) order_: Option<Vec<OrderBuilder>>,
    pub(crate) limit_: Option<u64>,
    pub(crate) offset_: Option<u64>,
}


#[derive(Clone, Default)]
pub struct JoinQueryBuilder {
    pub(crate) table: TableBuilder,
    pub(crate) typ: JoinTableType,
    pub(crate) filter: Option<QueryFilterBuilder>,
}


#[derive(Clone, Default)]
pub enum JoinTableType {
    Inner,
    Left,
    Right,
    Full,
}


#[derive(Clone, Default)]
pub struct QueryFilterBuilder {
    pub(crate) typ: QueryFilterType,
    pub(crate) filters: Vec<QueryFilterItemBuilder>,
}


#[derive(Clone, Default)]
pub enum QueryFilterType {
    And,
    Or,
}


#[derive(Clone, Default)]
pub enum QueryFilterItemBuilder {
    Simple(QuerySimpleFilterItemBuilder),
    Nested(QueryFilterBuilder),
}


#[derive(Clone, Default)]
pub struct QuerySimpleFilterItemBuilder {
    pub(crate) filter_typ_: QueryFilterType,
    pub(crate) field_: FilterFieldBuilder,
    pub(crate) value_: String,
    pub(crate) op_: FilterOperatorType,
}


#[derive(Clone, Default)]
pub enum FilterFieldBuilder {
    String(String),
    Dynamic(DynamicFilterFieldBuilder),
}


#[derive(Clone, Default)]
pub enum DynamicFilterFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(QueryBuilder),
}


#[derive(Clone, Default)]
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


#[derive(Clone, Default)]
pub struct OrderBuilder {
    pub(crate) field: String,
    pub(crate) typ: OrderType,
}


#[derive(Clone, Default)]
pub enum OrderFieldBuilder {
    String(String),
    Dynamic(DynamicOrderFieldBuilder),
}


#[derive(Clone, Default)]
pub enum DynamicOrderFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(FilterFieldBuilder),
}


#[derive(Clone, Default)]
pub enum OrderType {
    Asc,
    Desc,
}


#[derive(Clone, Default)]
pub struct UpdateBuilder {
    pub(crate) set_: Vec<UpdateSetFieldBuilder>,
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) join_: Vec<JoinQueryBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
}


#[derive(Clone, Default)]
pub struct UpdateSetFieldBuilder {
    pub(crate) field_: FilterFieldBuilder,
    pub(crate) value_: String,
}


#[derive(Clone, Default)]
pub struct DeleteBuilder {
    pub(crate) from_: Vec<TableBuilder>,
    pub(crate) filter_: Option<QueryFilterBuilder>,
}


#[derive(Clone, Default)]
pub struct InsertBuilder {
    pub(crate) into_: TableBuilder,
    pub(crate) fields_: Option<Vec<String>>,
    pub(crate) values_: Option<Vec<String>>,
    pub(crate) query_: Option<QueryBuilder>,
}