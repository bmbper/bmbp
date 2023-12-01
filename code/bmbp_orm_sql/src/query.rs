/// 查询构建器
/// 暂时支持PG，后续适配MYSQL\SQLite\SQLServer等

///SQL构建的抽象
pub trait SqlDynamicBuilder {
    fn build(&self) -> String;
}

#[derive(Clone)]
pub enum SelectBuilder {
    String(String),
    // 字段名 | 字段名 AS 别名 | 表名.字段名 | 表名.字段名 AS 别名 | SQL语句 | SQL语句 AS 别名 | 批量
    Dynamic(DynamicSelectBuilder),// 动态字段
}
impl SqlDynamicBuilder for SelectBuilder {
    fn build(&self) -> String {
        match self {
            SelectBuilder::String(s) => s.clone(),
            SelectBuilder::Dynamic(d) => d.build(),
        }
    }
}

#[derive(Clone)]
pub struct DynamicSelectBuilder {
    table_alias_: Option<String>,
    // 表别名
    field_: DynamicSelectBuilderType,
    // 动态字段
    alias_: Option<String>, // 字段别名
}
impl SqlDynamicBuilder for DynamicSelectBuilder {
    fn build(&self) -> String {
        let mut s = "".to_string();
        if let Some(ref table_alias) = self.table_alias_ {
            s += &format!("{} ", table_alias);
        }
        s += &self.field_.build();
        if let Some(ref alias) = self.alias_ {
            s += &format!(" AS {}", alias);
        }
        s
    }
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
impl SqlDynamicBuilder for DynamicSelectBuilderType {
    fn build(&self) -> String {
        match self {
            DynamicSelectBuilderType::String(s) => s.clone(),
            DynamicSelectBuilderType::Dynamic(d) => d.build(),
            DynamicSelectBuilderType::TempTable(t) => t.build(),
        }
    }
}

#[derive(Clone, Default)]
pub enum TableBuilder {
    String(String),
    Dynamic(DynamicTableBuilder),
}
impl SqlDynamicBuilder for TableBuilder {
    fn build(&self) -> String {
        match self {
            TableBuilder::String(s) => s.clone(),
            TableBuilder::Dynamic(d) => d.build(),
        }
    }
}

#[derive(Clone, Default)]
pub struct DynamicTableBuilder {
    // 数据库实例
    schema: Option<String>,
    // 表名
    table: DynamicTableBuilderType,
    // 表别名
    alias: Option<String>,
}
impl SqlDynamicBuilder for DynamicTableBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        if let Some(ref schema) = self.schema {
            sql += &format!("{}.", schema);
        }
        if let Some(ref alias) = self.alias {
            sql += &format!("{} AS {}, ", self.table.build(), alias);
        } else {
            sql += &format!("{}, ", self.table.build());
        }
        sql
    }
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
impl SqlDynamicBuilder for DynamicTableBuilderType {
    fn build(&self) -> String {
        match self {
            DynamicTableBuilderType::String(s) => s.clone(),
            DynamicTableBuilderType::Dynamic(d) => d.build(),
            DynamicTableBuilderType::TempTable(t) => t.build(),
        }
    }
}

#[derive(Clone, Default)]
pub struct QueryBuilder {
    select_: Vec<SelectBuilder>,
    from_: Vec<TableBuilder>,
    join_: Vec<JoinQueryBuilder>,
    filter_: Option<QueryFilterBuilder>,
    group_: Option<Vec<QueryFilterItemBuilder>>,
    order_: Option<Vec<OrderBuilder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
}
impl SqlDynamicBuilder for QueryBuilder {
    fn build(&self) -> String {
        let mut sql = "SELECT ".to_string();
        sql += &self.select_.iter().map(|s| s.build()).collect::<Vec<_>>().join(", ");
        sql += &self.from_.iter().map(|t| t.build()).collect::<Vec<_>>().join(", ");
        if!self.join_.is_empty() {
            sql += &self.join_.iter().map(|j| j.build()).collect::<Vec<_>>().join(" ");
        }
        if let Some(ref filter) = self.filter_ {
            sql += &filter.build();
        }
        if let Some(ref group) = self.group_ {
            sql += &group.iter().map(|g| g.build()).collect::<Vec<_>>().join(", ");
        }
        if let Some(ref order) = self.order_ {
            sql += &order.iter().map(|o| o.build()).collect::<Vec<_>>().join("");
        }
        sql
    }
}
#[derive(Clone, Default)]
pub struct JoinQueryBuilder {
    table: TableBuilder,
    typ: JoinTableType,
    filter: Option<QueryFilterBuilder>,
}
impl SqlDynamicBuilder for JoinQueryBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &format!("{} JOIN ", self.typ.to_string());
        sql += &self.table.build();
        if let Some(ref filter) = self.filter {
            sql += &filter.build();
        }
        sql
    }
}

#[derive(Clone, Default)]
pub enum JoinTableType {
    Inner,
    Left,
    Right,
    Full,
}
impl SqlDynamicBuilder for JoinTableType {
    fn build(&self) -> String {
        match self {
            JoinTableType::Inner => "INNER JOIN".to_string(),
            JoinTableType::Left => "LEFT JOIN".to_string(),
            JoinTableType::Right => "RIGHT JOIN".to_string(),
            JoinTableType::Full => "FULL JOIN".to_string(),
        }
    }
}

#[derive(Clone, Default)]
pub struct QueryFilterBuilder {
    typ: QueryFilterType,
    filters: Vec<QueryFilterItemBuilder>,
}

impl SqlDynamicBuilder for QueryFilterBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &format!("{} (", self.typ.to_string());
        sql += &self.filters.iter().map(|f| f.build()).collect::<Vec<_>>().join(", ");
        sql += &")";
        sql
    }
}
#[derive(Clone, Default)]
pub enum QueryFilterType {
    And,
    Or,
}

impl SqlDynamicBuilder for QueryFilterType {
    fn build(&self) -> String {
        match self {
            QueryFilterType::And => "AND".to_string(),
            QueryFilterType::Or => "OR".to_string(),
        }
    }
}
#[derive(Clone, Default)]
pub enum QueryFilterItemBuilder {
    Simple(QuerySimpleFilterItemBuilder),
    Nested(QueryFilterBuilder),
}
impl SqlDynamicBuilder for QueryFilterItemBuilder {
    fn build(&self) -> String {
        match self {
            QueryFilterItemBuilder::Simple(s) => s.build(),
            QueryFilterItemBuilder::Nested(n) => n.build(),
        }
    }
}
#[derive(Clone, Default)]
pub struct QuerySimpleFilterItemBuilder {
    filter_typ_: QueryFilterType,
    field_: FilterFieldBuilder,
    value_: String,
    op_: FilterOperatorType,
}
impl SqlDynamicBuilder for QuerySimpleFilterItemBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &format!("{} ", self.filter_typ_.to_string());
        sql += &self.field_.build();
        sql += &format!(" {} ", self.op_.to_string());
        sql += &self.value_;
        sql
    }
}
#[derive(Clone, Default)]
pub enum FilterFieldBuilder {
    String(String),
    Dynamic(DynamicFilterFieldBuilder),
}
impl SqlDynamicBuilder for FilterFieldBuilder {
    fn build(&self) -> String {
        match self {
            FilterFieldBuilder::String(s) => s.clone(),
            FilterFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}
#[derive(Clone, Default)]
pub enum DynamicFilterFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(QueryBuilder),
}
impl SqlDynamicBuilder for DynamicFilterFieldBuilder {
    fn build(&self) -> String {
        match self {
            DynamicFilterFieldBuilder::String(s) => s.clone(),
            DynamicFilterFieldBuilder::Dynamic(d) => d.build(),
        }
    }
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
impl SqlDynamicBuilder for FilterOperatorType {
    fn build(&self) -> String {
        match self {
            FilterOperatorType::Eq => "=".to_string(),
            FilterOperatorType::Ne => "!=".to_string(),
            FilterOperatorType::Gt => ">".to_string(),
            FilterOperatorType::Lt => "<".to_string(),
            FilterOperatorType::Ge => ">=".to_string(),
            FilterOperatorType::Le => "<=".to_string(),
            FilterOperatorType::Like => "LIKE".to_string(),
            FilterOperatorType::NotLike => "NOT LIKE".to_string(),
            FilterOperatorType::In => "IN".to_string(),
            FilterOperatorType::NotIn => "NOT IN".to_string(),
            FilterOperatorType::IsNull => "IS NULL".to_string(),
            FilterOperatorType::IsNotNull => "IS NOT NULL".to_string(),
        }
    }
}
#[derive(Clone, Default)]
pub struct OrderBuilder {
    field: String,
    typ: OrderType,
}
impl SqlDynamicBuilder for OrderBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &self.field;
        sql += &format!(" {}", self.typ.to_string());
        sql
    }
}
#[derive(Clone, Default)]
pub enum  OrderFieldBuilder {
   String(String),
   Dynamic(DynamicOrderFieldBuilder)
}
impl SqlDynamicBuilder for OrderFieldBuilder {
    fn build(&self) -> String {
        match self {
            OrderFieldBuilder::String(s) => s.clone(),
            OrderFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}
#[derive(Clone, Default)]
pub enum DynamicOrderFieldBuilder {
    // 字段名  | 表名.字段名 | SQL语句 |
    String(String),
    // 动态字段
    Dynamic(FilterFieldBuilder),
}
impl SqlDynamicBuilder for DynamicOrderFieldBuilder {
    fn build(&self) -> String {
        match self {
            DynamicOrderFieldBuilder::String(s) => s.clone(),
            DynamicOrderFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}
#[derive(Clone, Default)]
pub enum OrderType {
    Asc,
    Desc,
}
impl SqlDynamicBuilder for OrderType {
    fn build(&self) -> String {
        match self {
            OrderType::Asc => "ASC".to_string(),
            OrderType::Desc => "DESC".to_string(),
        }
    }
}
#[derive(Clone, Default)]
pub struct UpdateBuilder {
    set_: Vec<UpdateSetFieldBuilder>,
    from_: Vec<TableBuilder>,
    join_: Vec<JoinQueryBuilder>,
    filter_: Option<QueryFilterBuilder>,
}
impl SqlDynamicBuilder for UpdateBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &"UPDATE ".to_string();
        sql += &self.from_.iter().map(|f| f.build()).collect::<Vec<_>>().join(", ");
        sql += &" SET ".to_string();
        sql += &self.set_.iter().map(|f| f.build()).collect::<Vec<_>>().join(", ");
        if let Some(f) = &self.filter_ {
            sql += &" WHERE ".to_string();
            sql += &f.build();
        }
        sql
    }
}
#[derive(Clone, Default)]
pub struct UpdateSetFieldBuilder {
    field_: FilterFieldBuilder,
    value_: String,
}
impl SqlDynamicBuilder for UpdateSetFieldBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &self.field_.build();
        sql += &" = ";
        sql += &self.value_;
        sql
    }
}
#[derive(Clone, Default)]
pub struct DeleteBuilder {
    from_: Vec<TableBuilder>,
    filter_: Option<QueryFilterBuilder>,
}
impl SqlDynamicBuilder for DeleteBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &"DELETE FROM ".to_string();
        sql += &self.from_.iter().map(|f| f.build()).collect::<Vec<_>>().join(", ");
        if let Some(f) = &self.filter_ {
            sql += &" WHERE ".to_string();
            sql += &f.build();
        }
        sql
    }
}
#[derive(Clone, Default)]
pub struct InsertBuilder {
    into_: TableBuilder,
    fields_: Option<Vec<String>>,
    values_: Option<Vec<String>>,
    query_: Option<QueryBuilder>,
}
impl SqlDynamicBuilder for InsertBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &"INSERT INTO ".to_string();
        sql += &self.into_.build();
        if let Some(f) = &self.fields_ {
            sql += &" (".to_string();
            sql += &f.join(", ");
            sql += &") ";
        }
        if let Some(v) = &self.values_ {
            sql += &" VALUES (".to_string();
            sql += &v.join(", ");
            sql += &") ";
        }
        if let Some(q) = &self.query_ {
            sql += &q.build();
        }
        sql
    }
}
#[cfg(test)]
mod tests {}