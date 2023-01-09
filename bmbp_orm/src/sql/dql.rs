use serde_json::{Number, Value};

use bmbp_util::{camel_to_snake, snake_to_camel};

#[derive(Clone)]
pub struct QuerySQL {
    select: Vec<SelectField>,
    table: Vec<Table>,
    filter: Option<QueryFilter>,
    order: Vec<OrderField>,
    group: Vec<SelectField>,
}

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
    pub fn simple_filter() -> SimpleFilterInner {
        SimpleFilterInner {
            typ: FilterType::AND,
            fields: vec![],
            express: None,
        }
    }
    pub fn simple_filter_or() -> SimpleFilterInner {
        SimpleFilterInner {
            typ: FilterType::OR,
            fields: vec![],
            express: None,
        }
    }
    pub fn simple_filter_express(express: String) -> SimpleFilterInner {
        SimpleFilterInner {
            typ: FilterType::Express,
            fields: vec![],
            express: Some(express),
        }
    }
    pub fn complex_filter() -> ComplexFilterInner {
        ComplexFilterInner {
            typ: FilterType::AND,
            cmp: vec![],
            express: None,
        }
    }
    pub fn complex_filter_or() -> ComplexFilterInner {
        ComplexFilterInner {
            typ: FilterType::OR,
            cmp: vec![],
            express: None,
        }
    }
    pub fn complex_filter_express(express: String) -> ComplexFilterInner {
        ComplexFilterInner {
            typ: FilterType::Express,
            cmp: vec![],
            express: Some(express),
        }
    }
}

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

impl QuerySQL {
    // select_c_as_df 默认返回列別名
    pub fn select_c_as_df(&mut self, column: String) -> &mut Self {
        let alias = snake_to_camel(column.clone());
        self.select
            .push(SelectField::COLUMN(ColumnFieldInner::new_as(column, alias)));
        self
    }

    pub fn select_c_as(&mut self, column: String, alias: String) -> &mut Self {
        self.select
            .push(SelectField::COLUMN(ColumnFieldInner::new_as(column, alias)));
        self
    }

    pub fn select(&mut self, field: String) -> &mut Self {
        self.select
            .push(SelectField::COLUMN(ColumnFieldInner::new(field)));
        self
    }

    pub fn select_as(&mut self, field: String, alias: String) -> &mut Self {
        self.select
            .push(SelectField::COLUMN(ColumnFieldInner::new_as(field, alias)));

        self
    }

    pub fn select_extend(&mut self, fields: Vec<SelectField>) -> &mut Self {
        self.select.extend_from_slice(fields.as_slice());
        self
    }

    pub fn from(&mut self, table: String) -> &mut Self {
        self.table.push(Table::new(table));
        self
    }

    pub fn from_as(&mut self, table: String, alias: String) -> &mut Self {
        self.table.push(Table::new_as(table, alias));
        self
    }

    pub fn order_asc(&mut self, field: String) -> &mut Self {
        self.order.push(OrderField::asc(field));
        self
    }

    pub fn order_desc(&mut self, field: String) -> &mut Self {
        self.order.push(OrderField::desc(field));
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
            self.filter = Some(QueryFilter::simple());
        }
        self.filter.as_mut().unwrap()
    }
}

/// 简易查询过虑器
impl QuerySQL {
    // 简易SQL
    pub fn simple_filter_inner(&mut self) -> &mut SimpleFilterInner {
        if self.filter.is_none() || self.filter.as_ref().unwrap().is_complex() {
            self.filter = Some(QueryFilter::simple());
        }
        self.filter.as_mut().unwrap().as_simple().unwrap()
    }

    pub fn s_f_eq(&mut self, field: String) -> &mut Self {
        self.simple_filter_inner().s_f_eq(field);
        self
    }
    pub fn s_c_eq(&mut self, column: String) -> &mut Self {
        self.simple_filter_inner().s_c_eq(column);
        self
    }
    pub fn s_f_eq_as(&mut self, field: String, value_as: String) -> &mut Self {
        self.simple_filter_inner().s_f_eq_as(field, value_as);
        self
    }
    pub fn s_c_eq_as(&mut self, column: String, value_as: String) -> &mut Self {
        self.simple_filter_inner().s_c_eq_as(column, value_as);
        self
    }

    pub fn r_f_eq_string(&mut self, field: String, value: String) -> &mut Self {
        self.simple_filter_inner().r_f_eq_string(field, value);
        self
    }

    pub fn r_c_eq_string(&mut self, column: String, value: String) -> &mut Self {
        self.simple_filter_inner().r_f_eq_string(column, value);
        self
    }

    pub fn r_f_eq_bool(&mut self, field: String, value: bool) -> &mut Self {
        self.simple_filter_inner().r_f_eq_bool(field, value);
        self
    }

    pub fn r_c_eq_bool(&mut self, column: String, value: bool) -> &mut Self {
        self.simple_filter_inner().r_c_eq_bool(column, value);
        self
    }

    pub fn r_f_eq_isize(&mut self, field: String, value: isize) -> &mut Self {
        self.simple_filter_inner().r_f_eq_isize(field, value);
        self
    }

    pub fn r_c_eq_isize(&mut self, column: String, value: isize) -> &mut Self {
        self.simple_filter_inner().r_c_eq_isize(column, value);
        self
    }
    pub fn r_f_eq_f64(&mut self, field: String, value: f64) -> &mut Self {
        self.simple_filter_inner().r_f_eq_f64(field, value);
        self
    }

    pub fn r_c_eq_f64(&mut self, column: String, value: f64) -> &mut Self {
        self.simple_filter_inner().r_c_eq_f64(column, value);
        self
    }
}

impl QuerySQL {
    pub fn s_f_rlk(&mut self, field: String) -> &mut Self {
        self.simple_filter_inner().s_f_rlk(field);
        self
    }
}

#[derive(Clone)]
pub enum SelectField {
    COLUMN(ColumnFieldInner),
    CST(CstFieldInner),
    FUNC(FuncFieldInner),
    EXPRESS(ExpressFieldInner),
    SQL(SqlSelectFieldInner),
}

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

#[derive(Clone)]
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
#[derive(Clone)]
pub struct ColumnFieldInner {
    table_alias: String,
    tag: Option<SelectType>,
    field: String,
    alias: String,
}

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
#[derive(Clone)]
pub struct CstFieldInner {
    tag: Option<SelectType>,
    field: String,
    alias: String,
}

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
#[derive(Clone)]
pub struct FuncFieldInner {
    typ: Option<SelectType>,
    func: String,
    field: Vec<String>,
    connect: String,
}

/// ExpressFieldInner 表达式组合列
#[derive(Clone)]
pub struct ExpressFieldInner {
    typ: Option<SelectType>,
    express: String,
    field: Vec<String>,
}

/// SqlSelectFieldInner 查询返回SQL子语句
#[derive(Clone)]
pub struct SqlSelectFieldInner {
    typ: Option<SelectType>,
    field: QuerySQL,
    alias: String,
}

#[derive(Clone)]
pub struct Table {
    name: String,
    alias: String,
    join: Option<JoinTable>,
}

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

#[derive(Clone)]
pub struct JoinTable {
    select: Vec<SelectField>,
    name: String,
    alias: String,
    filter: QueryFilter,
}

#[derive(Clone)]
pub enum QueryFilter {
    Complex(ComplexFilterInner),
    Simple(SimpleFilterInner),
}

impl QueryFilter {
    pub fn simple() -> QueryFilter {
        QueryFilter::Simple(SimpleFilterInner::and())
    }
    pub fn simple_or() -> QueryFilter {
        QueryFilter::Simple(SimpleFilterInner::or())
    }
    pub fn simple_express(express: String) -> SimpleFilterInner {
        SimpleFilterInner::express(express)
    }
    pub fn complex() -> ComplexFilterInner {
        ComplexFilterInner::and()
    }
    pub fn complex_or() -> ComplexFilterInner {
        ComplexFilterInner::or()
    }
    pub fn complex_express(express: String) -> ComplexFilterInner {
        ComplexFilterInner::express(express)
    }

    pub fn as_simple(&mut self) -> Option<&mut SimpleFilterInner> {
        match self {
            QueryFilter::Simple(simple) => Some(simple),
            QueryFilter::Complex(_) => None,
        }
    }
    pub fn is_simple(&mut self) -> bool {
        match self {
            QueryFilter::Simple(_) => true,
            QueryFilter::Complex(_) => false,
        }
    }
    pub fn as_complex(&mut self) -> Option<&mut ComplexFilterInner> {
        match self {
            QueryFilter::Simple(_) => None,
            QueryFilter::Complex(complex) => Some(complex),
        }
    }
    pub fn is_complex(&self) -> bool {
        match self {
            QueryFilter::Simple(_) => false,
            QueryFilter::Complex(_) => true,
        }
    }
}

#[derive(Clone)]
pub struct ComplexFilterInner {
    typ: FilterType,
    cmp: Vec<QueryFilter>,
    express: Option<String>,
}

impl ComplexFilterInner {
    pub fn and() -> Self {
        ComplexFilterInner {
            typ: FilterType::AND,
            cmp: vec![],
            express: None,
        }
    }
    pub fn or() -> Self {
        ComplexFilterInner {
            typ: FilterType::OR,
            cmp: vec![],
            express: None,
        }
    }
    pub fn express(express: String) -> Self {
        ComplexFilterInner {
            typ: FilterType::Express,
            cmp: vec![],
            express: Some(express),
        }
    }
}

#[derive(Clone)]
pub struct SimpleFilterInner {
    typ: FilterType,
    fields: Vec<FilterField>,
    express: Option<String>,
}

impl SimpleFilterInner {
    pub fn and() -> Self {
        SimpleFilterInner {
            typ: FilterType::AND,
            fields: vec![],
            express: None,
        }
    }
    pub fn or() -> Self {
        SimpleFilterInner {
            typ: FilterType::OR,
            fields: vec![],
            express: None,
        }
    }
    pub fn express(express: String) -> Self {
        SimpleFilterInner {
            typ: FilterType::Express,
            fields: vec![],
            express: Some(express),
        }
    }
}

impl SimpleFilterInner {
    pub fn get_filter_type(&self) -> &FilterType {
        &self.typ
    }

    pub fn set_filter_type(&mut self, filter_type: FilterType) -> &mut Self {
        self.typ = filter_type;
        self
    }

    pub fn get_field_slice(&self) -> &[FilterField] {
        self.fields.as_slice()
    }

    pub fn get_mut_field_slice(&mut self) -> &mut [FilterField] {
        self.fields.as_mut_slice()
    }

    pub fn set_fields(&mut self, fields: Vec<FilterField>) -> &mut Self {
        self.fields = fields;
        self
    }

    pub fn set_express(&mut self, express: String) -> &mut Self {
        self.express = Some(express);
        self
    }

    pub fn get_express(&self) -> Option<&String> {
        self.express.as_ref()
    }

    pub fn get_raw_express(&self) -> String {
        if self.express.is_none() {
            "".to_string()
        } else {
            self.express.as_ref().unwrap().clone()
        }
    }
}

impl SimpleFilterInner {
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

// like
impl SimpleFilterInner {
    pub(crate) fn s_f_rlk(&mut self, field: String) -> &mut Self {
        self.fields
            .push(FilterField::s_f_cmp(CompareType::RLK, field));
        self
    }
}

impl SimpleFilterInner {}

impl SimpleFilterInner {}

impl SimpleFilterInner {}

#[derive(Clone)]
pub enum FilterType {
    AND,
    OR,
    Express,
}

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::AND => "AND".to_string(),
            FilterType::OR => "OR".to_string(),
            FilterType::Express => "EXPRESS".to_string(),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum FilterValue {
    SCRIPT(String),
    POSITION(usize),
    VALUE(Value),
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FuncCompareFieldInner {}

#[derive(Clone)]
pub struct QueryComparefieldInner {}

#[derive(Clone)]
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

#[derive(Clone)]
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
    pub fn desc_vec(field: Vec<String>) -> Vec<Self> {
        vec![]
    }

    pub fn asc_vec(field: Vec<String>) -> Vec<Self> {
        vec![]
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

    pub fn asc_field_vec(field: Vec<SelectField>) -> Vec<Self> {
        vec![]
    }
    pub fn desc_field_vec(field: Vec<SelectField>) -> Vec<Self> {
        vec![]
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
