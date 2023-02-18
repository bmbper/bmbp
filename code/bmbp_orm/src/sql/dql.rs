use serde_json::{Number, Value};

use bmbp_util::{camel_to_snake, snake_to_camel};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct QuerySQL {
    select: Vec<SelectField>,
    table: Vec<Table>,
    filter: Option<QueryFilter>,
    order: Vec<OrderField>,
    group: Vec<SelectField>,
}
#[allow(dead_code)]
impl QuerySQL {
    pub fn new() -> QuerySQL {
        QuerySQL {
            select: vec![],
            table: vec![],
            filter: None,
            order: vec![],
            group: vec![],
        }
    }
}
#[allow(dead_code)]
impl QuerySQL {
    pub fn set_filter(&mut self, filter: QueryFilter) -> &mut Self {
        self.filter = Some(filter);
        self
    }
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter.as_ref()
    }
    pub fn get_select(&self) -> &[SelectField] {
        self.select.as_slice()
    }
    pub fn get_table(&self) -> &[Table] {
        self.table.as_slice()
    }
    pub fn get_order(&self) -> &[OrderField] {
        self.order.as_slice()
    }
    pub fn get_group(&self) -> &[SelectField] {
        &self.group.as_slice()
    }
}

#[allow(dead_code)]
impl QuerySQL {
    pub fn select_field(&mut self, field: String) -> &mut Self {
        let column = camel_to_snake(field.clone());
        self.select_column_as(column, field)
    }

    pub fn select_column(&mut self, column: String) -> &mut Self {
        let alias = snake_to_camel(column.clone());
        self.select_column_as(column, alias)
    }

    pub fn raw_select(&mut self, field: String) -> &mut Self {
        self.select_column_as(field.clone(), field)
    }

    pub fn select_field_as(&mut self, field: String, alias: String) -> &mut Self {
        let column = camel_to_snake(field.clone());
        self.select_column_as(column, alias)
    }

    pub fn select_column_as(&mut self, column: String, alias: String) -> &mut Self {
        self.select
            .push(SelectField::COLUMN(ColumnFieldInner::new_as(column, alias)));
        self
    }

    pub fn select(&mut self, field: String) -> &mut Self {
        self.select_field(field)
    }

    pub fn select_as(&mut self, field: String, alias: String) -> &mut Self {
        self.select_field_as(field, alias)
    }

    pub fn add_select(&mut self, select_field: SelectField) -> &mut Self {
        self.select.push(select_field);
        self
    }

    pub fn select_extend(&mut self, fields: Vec<SelectField>) -> &mut Self {
        self.select.extend_from_slice(fields.as_slice());
        self
    }

    pub fn target_table(&mut self, table: String) -> &mut Self {
        self.table.push(Table::new(table));
        self
    }

    pub fn from_as(&mut self, table: String, alias: String) -> &mut Self {
        self.table.push(Table::new_as(table, alias));
        self
    }

    pub fn order_field_asc(&mut self, field: String) -> &mut Self {
        let column = camel_to_snake(field.clone());
        self.order_column_asc(column)
    }

    pub fn order_field_desc(&mut self, field: String) -> &mut Self {
        let column = camel_to_snake(field.clone());
        self.order_column_desc(column)
    }

    pub fn order_column_asc(&mut self, column: String) -> &mut Self {
        self.order.push(OrderField::asc(column));
        self
    }

    pub fn order_column_desc(&mut self, column: String) -> &mut Self {
        self.order.push(OrderField::desc(column));
        self
    }

    pub fn group_by(&mut self, field: String) -> &mut Self {
        self.group.push(SelectField::column(field));
        self
    }

    pub fn group_by_vec(&mut self, field: Vec<String>) -> &mut Self {
        for item in field {
            self.group.push(SelectField::column(item));
        }
        self
    }

    pub fn group_by_field(&mut self, field: SelectField) -> &mut Self {
        self.group.push(field);
        self
    }

    pub fn group_by_field_vec(&mut self, field: Vec<SelectField>) -> &mut Self {
        self.group.extend_from_slice(&field);
        self
    }

    pub fn filter(&mut self) -> &mut QueryFilter {
        if self.filter.is_none() {
            self.filter = Some(QueryFilter::new());
        }
        self.filter.as_mut().unwrap()
    }
}

/// 简易查询过虑器
#[allow(dead_code)]
impl QuerySQL {
    // 简易SQL
    pub fn get_mut_filter(&mut self) -> &mut QueryFilter {
        if self.filter.is_none() {
            self.filter = Some(QueryFilter::new());
        }
        self.filter.as_mut().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SelectField {
    COLUMN(ColumnFieldInner),
    CST(CstFieldInner),
    FUNC(FuncFieldInner),
    EXPRESS(ExpressFieldInner),
    SQL(SqlSelectFieldInner),
}
#[allow(dead_code)]
impl SelectField {
    pub fn to_field(&self) -> String {
        "".to_string()
    }

    pub fn column(field: String) -> Self {
        SelectField::COLUMN(ColumnFieldInner::new(field))
    }

    pub fn column_as(field: String, alias: String) -> Self {
        SelectField::COLUMN(ColumnFieldInner::new_as(field, alias))
    }

    pub fn cst(field: String) -> Self {
        SelectField::CST(CstFieldInner::new(field))
    }

    pub fn cst_as(field: String, alias: String) -> Self {
        SelectField::CST(CstFieldInner::new_as(field, alias))
    }
}

#[derive(Clone, Debug)]
pub enum SelectType {
    DISTINCT,
}

impl SelectType {}

impl ToString for SelectType {
    fn to_string(&self) -> String {
        match self {
            SelectType::DISTINCT => "DISTINCT".to_string(),
        }
    }
}

/// ColumnFieldInner  返回列
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ColumnFieldInner {
    table_alias: String,
    tag: Option<SelectType>,
    field: String,
    alias: String,
}

#[allow(dead_code)]
impl ColumnFieldInner {
    pub fn new(field: String) -> Self {
        ColumnFieldInner {
            tag: None,
            field,
            alias: "".to_string(),
            table_alias: "".to_string(),
        }
    }

    pub fn new_as(field: String, alias: String) -> Self {
        ColumnFieldInner {
            tag: None,
            field,
            alias,
            table_alias: "".to_string(),
        }
    }

    pub fn table_alias(&self) -> &String {
        &self.table_alias
    }

    pub fn set_table_alias(&mut self, table_alias: String) -> &mut Self {
        self.table_alias = table_alias;
        self
    }

    pub fn set_tag(&mut self, tag: SelectType) -> &mut Self {
        self.tag = Some(tag);
        self
    }

    pub fn tag(&self) -> Option<&SelectType> {
        self.tag.as_ref()
    }

    pub fn set_field(&mut self, field: String) -> &mut Self {
        self.field = field;
        self
    }

    pub fn field(&self) -> &String {
        &self.field
    }
    pub fn set_alias(&mut self, alias: String) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn alias(&self) -> &String {
        &self.alias
    }
}

/// ConstFieldInner 常量列
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CstFieldInner {
    tag: Option<SelectType>,
    field: String,
    alias: String,
}
#[allow(dead_code)]
impl CstFieldInner {
    pub fn new(field: String) -> Self {
        CstFieldInner {
            tag: None,
            field,
            alias: "".to_string(),
        }
    }

    pub fn new_as(field: String, alias: String) -> Self {
        CstFieldInner {
            tag: None,
            field: field,
            alias: alias,
        }
    }

    pub fn set_alias(&mut self, alias: String) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn set_field(&mut self, field: String) -> &mut Self {
        self.field = field;
        self
    }

    pub fn set_tag(&mut self, tag: SelectType) -> &mut Self {
        self.tag = Some(tag);
        self
    }

    pub fn tag(&self) -> Option<&SelectType> {
        self.tag.as_ref()
    }

    pub fn get_field(&self) -> &String {
        &self.field
    }

    pub fn get_alias(&self) -> &String {
        &self.alias
    }
}

/// FuncFieldInner 简单函数列
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FuncFieldInner {
    typ: Option<SelectType>,
    func: String,
    field: Vec<String>,
    connect: String,
}

/// ExpressFieldInner 表达式组合列
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExpressFieldInner {
    typ: Option<SelectType>,
    express: String,
    field: Vec<String>,
}

/// SqlSelectFieldInner 查询返回SQL子语句
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SqlSelectFieldInner {
    typ: Option<SelectType>,
    field: QuerySQL,
    alias: String,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Table {
    name: String,
    alias: String,
    join: Option<JoinTable>,
}

#[allow(dead_code)]
impl Table {
    pub fn new(table: String) -> Self {
        Table {
            name: table,
            alias: "".to_string(),
            join: None,
        }
    }
    pub fn new_as(table: String, alias: String) -> Self {
        Table {
            name: table,
            alias,
            join: None,
        }
    }

    pub fn set_table_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_table_alias(&mut self, alias: String) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn set_join(&mut self, join: JoinTable) -> &mut JoinTable {
        self.join = Some(join);
        self.join.as_mut().unwrap()
    }

    pub fn table_name(&self) -> &String {
        &self.name
    }
    pub fn table_alias(&self) -> &String {
        &self.alias
    }
    pub fn join(&self) -> Option<&JoinTable> {
        self.join.as_ref()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct JoinTable {
    select: Vec<SelectField>,
    name: String,
    alias: String,
    filter: QueryFilter,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct QueryFilter {
    typ: FilterType,
    fields: Vec<FilterField>,
}

impl QueryFilter {
    pub fn new() -> QueryFilter {
        QueryFilter {
            typ: FilterType::AND,
            fields: vec![],
        }
    }
    pub fn new_or() -> QueryFilter {
        QueryFilter {
            typ: FilterType::OR,
            fields: vec![],
        }
    }
    pub fn new_express(express: String) -> QueryFilter {
        QueryFilter {
            typ: FilterType::Express(express),
            fields: vec![],
        }
    }
}

impl QueryFilter {
    pub fn get_filter_type(&self) -> &FilterType {
        &self.typ
    }
    pub fn get_field_slice(&self) -> &[FilterField] {
        self.fields.as_slice()
    }
}

impl QueryFilter {
    pub fn s_f_eq(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::EQ, field));
        self
    }

    pub fn s_c_eq(&mut self, column: String) -> &mut Self {
        self.fields
            .push(FilterField::s_c_cmp(CompareType::EQ, column));
        self
    }

    pub fn s_f_eq_as(&mut self, field: String, value_as: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp_as(CompareType::EQ, field, value_as));
        self
    }

    pub fn s_c_eq_as(&mut self, column: String, value_as: String) -> &mut Self {
        self.fields
            .push(FilterField::s_c_cmp_as(CompareType::EQ, column, value_as));
        self
    }

    pub fn r_f_eq_string(&mut self, field: String, value: String) -> &mut Self {
        self.fields
            .push(FilterField::r_f_cmp_string(CompareType::EQ, field, value));
        self
    }

    pub fn r_c_eq_string(&mut self, column: String, value: String) -> &mut Self {
        self.fields
            .push(FilterField::r_c_cmp_string(CompareType::EQ, column, value));
        self
    }

    pub fn r_f_eq_bool(&mut self, field: String, value: bool) -> &mut Self {
        self.fields
            .push(FilterField::r_f_cmp_bool(CompareType::EQ, field, value));
        self
    }

    pub fn r_c_eq_bool(&mut self, column: String, value: bool) -> &mut Self {
        self.fields
            .push(FilterField::r_c_cmp_bool(CompareType::EQ, column, value));
        self
    }

    pub fn r_f_eq_isize(&mut self, field: String, value: isize) -> &mut Self {
        let number_value = Value::Number(Number::from(value));
        self.fields.push(FilterField::r_f_cmp_value(
            CompareType::EQ,
            field,
            number_value,
        ));
        self
    }

    pub fn r_c_eq_isize(&mut self, column: String, value: isize) -> &mut Self {
        let number_value = Value::Number(Number::from(value));
        self.fields.push(FilterField::r_c_cmp_value(
            CompareType::EQ,
            column,
            number_value,
        ));
        self
    }
    pub fn r_f_eq_f64(&mut self, field: String, value: f64) -> &mut Self {
        let number_value = Value::Number(Number::from_f64(value).unwrap());
        self.fields.push(FilterField::r_f_cmp_value(
            CompareType::EQ,
            field,
            number_value,
        ));
        self
    }

    pub fn r_c_eq_f64(&mut self, column: String, value: f64) -> &mut Self {
        let number_value = Value::Number(Number::from_f64(value).unwrap());
        self.fields.push(FilterField::r_c_cmp_value(
            CompareType::EQ,
            column,
            number_value,
        ));
        self
    }
}

// LIKE
impl QueryFilter {
    pub fn s_f_lk(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::LK, field));
        self
    }
    pub fn s_f_llk(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::LLK, field));
        self
    }

    pub fn s_f_rlk(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::RLK, field));
        self
    }
}

#[allow(dead_code)]
impl QueryFilter {
    pub fn s_f_in(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::IN, field));
        self
    }
    pub fn s_f_nin(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::NIN, field));
        self
    }
}

#[derive(Clone, Debug)]
pub enum FilterType {
    AND,
    OR,
    Express(String),
}

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::AND => "AND".to_string(),
            FilterType::OR => "OR".to_string(),
            FilterType::Express(_) => "EXPRESS".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CompareType {
    EQ,
    // equal
    NEQ,
    // not equal
    GT,
    // great than
    GET,
    // great equal than
    LT,
    // less than
    LET,
    // less equal than
    LK,
    // like
    NLK,
    // not like
    RLK,
    // like '_%'
    NRLK,
    // not like '_%'
    LLK,
    // like '%_'
    NLLK,
    // NOT like '%_'
    ISN,
    // is null
    ISNN,
    // is not null
    LIMIT,
    // limit
    OFFSET,
    // offset
    EX(QuerySQL),
    // exists
    NEX(QuerySQL),
    // not exists
    IN,
    // in
    NIN,
    // not in
    INSQL(QuerySQL),
    NINSQL(QuerySQL),
}

impl ToString for CompareType {
    fn to_string(&self) -> String {
        match self {
            CompareType::EQ => "=".to_string(),
            CompareType::NEQ => "!=".to_string(),
            CompareType::GT => ">".to_string(),
            CompareType::GET => ">=".to_string(),
            CompareType::LT => "<".to_string(),
            CompareType::LET => "<=".to_string(),
            CompareType::LK => "LIKE".to_string(),
            CompareType::NLK => "NOT LIKE".to_string(),
            CompareType::RLK => "LIKE".to_string(),
            CompareType::NRLK => "NOT LIKE".to_string(),
            CompareType::LLK => "LIKE".to_string(),
            CompareType::NLLK => "NOT LIKE".to_string(),
            CompareType::ISN => "IS NULL".to_string(),
            CompareType::ISNN => "IS NOT NULL".to_string(),
            CompareType::LIMIT => "LIMIT".to_string(),
            CompareType::OFFSET => "OFFSET".to_string(),
            CompareType::EX(_) => "EXISTS".to_string(),
            CompareType::NEX(_) => "NOT EXISTS".to_string(),
            CompareType::IN => "IN".to_string(),
            CompareType::NIN => "NOT IN".to_string(),
            CompareType::INSQL(_) => "IN".to_string(),
            CompareType::NINSQL(_) => "NOT IN".to_string(),
        }
    }
}

impl CompareType {
    pub fn is_simple(&self) -> bool {
        match self {
            CompareType::LK
            | CompareType::NLK
            | CompareType::RLK
            | CompareType::NRLK
            | CompareType::LLK
            | CompareType::NLLK
            | CompareType::IN
            | CompareType::NIN
            | CompareType::INSQL(_)
            | CompareType::NINSQL(_)
            | CompareType::EX(_)
            | CompareType::NEX(_) => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            CompareType::ISN | CompareType::ISNN => true,
            _ => false,
        }
    }

    pub fn is_like(&self) -> bool {
        match self {
            CompareType::LK
            | CompareType::NLK
            | CompareType::RLK
            | CompareType::NRLK
            | CompareType::LLK
            | CompareType::NLLK => true,
            _ => false,
        }
    }

    pub fn is_in(&self) -> bool {
        match self {
            CompareType::IN | CompareType::NIN => true,
            _ => false,
        }
    }

    pub fn is_in_sql(&self) -> bool {
        match self {
            CompareType::INSQL(_) | CompareType::NINSQL(_) => true,
            _ => false,
        }
    }

    pub fn is_exists(&self) -> bool {
        match self {
            CompareType::EX(_) | CompareType::NEX(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum FilterValue {
    SCRIPT(String),
    POSITION(usize),
    VALUE(Value),
    Query(QuerySQL),
    Filter(QueryFilter),
}

#[derive(Clone, Debug)]
pub struct FilterField {
    cp: CompareType,
    column: CompareField,
    value: FilterValue,
}

impl FilterField {
    pub fn get_cp(&self) -> &CompareType {
        &self.cp
    }
    pub fn get_column(&self) -> &CompareField {
        &self.column
    }
    pub fn get_value(&self) -> &FilterValue {
        &self.value
    }
}

impl FilterField {
    pub fn s_f_cmp(cmp_type: CompareType, field: String) -> FilterField {
        let column = camel_to_snake(field.clone());
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::SCRIPT(field),
        }
    }
    pub fn s_c_cmp(cmp_type: CompareType, column: String) -> FilterField {
        let field = snake_to_camel(column.clone());
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::SCRIPT(field),
        }
    }

    pub fn s_f_cmp_as(cmp_type: CompareType, field: String, value_as: String) -> FilterField {
        let column = camel_to_snake(field.clone());
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::SCRIPT(value_as),
        }
    }
    pub fn s_c_cmp_as(cmp_type: CompareType, column: String, value_as: String) -> FilterField {
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::SCRIPT(value_as),
        }
    }

    pub fn r_f_cmp_string(cmp_type: CompareType, field: String, value: String) -> FilterField {
        let column = camel_to_snake(field);
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(Value::String(value)),
        }
    }

    pub fn r_c_cmp_string(cmp_type: CompareType, column: String, value: String) -> FilterField {
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(Value::String(value)),
        }
    }
    pub fn r_f_cmp_bool(cmp_type: CompareType, field: String, value: bool) -> FilterField {
        let column = camel_to_snake(field);
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(Value::Bool(value)),
        }
    }

    pub fn r_c_cmp_bool(cmp_type: CompareType, column: String, value: bool) -> FilterField {
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(Value::Bool(value)),
        }
    }
    pub fn r_f_cmp_value(cmp_type: CompareType, field: String, value: Value) -> FilterField {
        let column = camel_to_snake(field);
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(value),
        }
    }

    pub fn r_c_cmp_value(cmp_type: CompareType, column: String, value: Value) -> FilterField {
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(value),
        }
    }

    pub fn r_c_cmp(cmp_type: CompareType, column: String, value: String) -> FilterField {
        FilterField {
            cp: cmp_type,
            column: CompareField::Column(column),
            value: FilterValue::VALUE(Value::String(value)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CompareField {
    Column(String),
    Func(FuncCompareFieldInner),
    Query(QueryComparefieldInner),
}

impl CompareField {
    pub fn new(column: String) -> Self {
        CompareField::Column(column)
    }
}

#[derive(Clone, Debug)]
pub struct FuncCompareFieldInner {}

#[derive(Clone, Debug)]
pub struct QueryComparefieldInner {}

#[derive(Clone, Debug)]
pub enum OrderType {
    Asc,
    Desc,
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Asc => "ASC".to_string(),
            OrderType::Desc => "DESC".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderField {
    field: String,
    order_type: OrderType,
}

impl OrderField {
    pub fn asc(field: String) -> Self {
        OrderField {
            field,
            order_type: OrderType::Asc,
        }
    }
    pub fn desc(field: String) -> Self {
        OrderField {
            field,
            order_type: OrderType::Desc,
        }
    }
    pub fn desc_vec(fields: Vec<String>) -> Vec<Self> {
        let mut order_vec = vec![];
        for field in fields.as_slice() {
            order_vec.push(OrderField {
                field: field.clone(),
                order_type: OrderType::Desc,
            })
        }
        order_vec
    }

    pub fn asc_vec(fields: Vec<String>) -> Vec<Self> {
        let mut order_vec = vec![];
        for field in fields.as_slice() {
            order_vec.push(OrderField {
                field: field.clone(),
                order_type: OrderType::Asc,
            })
        }
        order_vec
    }

    pub fn asc_field(field: SelectField) -> Self {
        OrderField {
            field: field.to_field(),
            order_type: OrderType::Asc,
        }
    }

    pub fn desc_field(field: SelectField) -> Self {
        OrderField {
            field: field.to_field(),
            order_type: OrderType::Desc,
        }
    }

    pub fn asc_field_vec(fields: Vec<SelectField>) -> Vec<Self> {
        let mut order_vec = vec![];
        for field in fields.as_slice() {
            order_vec.push(OrderField {
                field: field.to_field(),
                order_type: OrderType::Asc,
            })
        }
        order_vec
    }
    pub fn desc_field_vec(fields: Vec<SelectField>) -> Vec<Self> {
        let mut order_vec = vec![];
        for field in fields.as_slice() {
            order_vec.push(OrderField {
                field: field.to_field(),
                order_type: OrderType::Desc,
            })
        }
        order_vec
    }
}

impl OrderField {
    pub fn get_field(&self) -> &String {
        &self.field
    }
    pub fn set_field(&mut self, field: String) -> &mut Self {
        self.field = field;
        self
    }
    pub fn get_order_type(&self) -> &OrderType {
        &self.order_type
    }
    pub fn set_order_type(&mut self, order_type: OrderType) -> &mut Self {
        self.order_type = order_type;
        self
    }
}
