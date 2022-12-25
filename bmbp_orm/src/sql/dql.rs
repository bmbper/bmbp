use case_style::{
    formats::{camel, CamelCase},
    CaseStyle,
};
use serde_json::{Number, Value};

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
            cmp: vec![],
            express: None,
        }
    }
    pub fn simple_filter_or() -> SimpleFilterInner {
        SimpleFilterInner {
            typ: FilterType::OR,
            cmp: vec![],
            express: None,
        }
    }
    pub fn simple_filter_express(express: String) -> SimpleFilterInner {
        SimpleFilterInner {
            typ: FilterType::Express,
            cmp: vec![],
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
    fn set_filter(&mut self, filter: QueryFilter) -> &mut Self {
        self.filter = Some(filter);
        self
    }
    pub fn get_select(&self) -> &[SelectField] {
        self.select.as_slice()
    }

    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter.as_ref()
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

    pub fn filter(&mut self, filter: SimpleFilterInner) -> &mut Self {
        self.filter = Some(QueryFilter::Simple(filter));
        self
    }

    pub fn filter_complex(&mut self, filter: ComplexFilterInner) -> &mut Self {
        self.filter = Some(QueryFilter::Complex(filter));
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
            field: field,
            alias: "".to_string(),
            table_alias: "".to_string(),
        }
    }

    pub fn new_as(field: String, alias: String) -> Self {
        ColumnFieldInner {
            tag: None,
            field: field,
            alias: alias,
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
            field: field,
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

    pub fn filed(&self) -> &String {
        &self.field
    }

    pub fn alias(&self) -> &String {
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
    pub fn simple() -> SimpleFilterInner {
        SimpleFilterInner::and()
    }

    pub fn simple_or() -> SimpleFilterInner {
        SimpleFilterInner::or()
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
    cmp: Vec<FilterField>,
    express: Option<String>,
}

impl SimpleFilterInner {
    pub fn and() -> Self {
        SimpleFilterInner {
            typ: FilterType::AND,
            cmp: vec![],
            express: None,
        }
    }
    pub fn or() -> Self {
        SimpleFilterInner {
            typ: FilterType::OR,
            cmp: vec![],
            express: None,
        }
    }
    pub fn express(express: String) -> Self {
        SimpleFilterInner {
            typ: FilterType::Express,
            cmp: vec![],
            express: Some(express),
        }
    }
}

impl SimpleFilterInner {
    pub fn eq_str(&mut self, field: String, value: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_str(CompareType::EQ, field, value));
        self
    }
    pub fn eq_int(&mut self, field: String, value: isize) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_int(CompareType::EQ, field, value));
        self
    }
    pub fn eq_f64(&mut self, field: String, value: f64) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_f64(CompareType::EQ, field, value));
        self
    }
    pub fn eq_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script(CompareType::EQ, field));
        self
    }
    pub fn eq_raw_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script(CompareType::EQ, field));
        self
    }
    pub fn eq_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_camcal(CompareType::EQ, field));
        self
    }
    pub fn eq_raw_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script_camcal(CompareType::EQ, field));
        self
    }
    pub fn eq_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_as(CompareType::EQ, field, alias));
        self
    }
    pub fn eq_raw_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp.push(FilterField::cmp_raw_script_as(
            CompareType::EQ,
            field,
            alias,
        ));
        self
    }
}

impl SimpleFilterInner {
    pub fn neq_str(&mut self, field: String, value: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_str(CompareType::NEQ, field, value));
        self
    }
    pub fn neq_int(&mut self, field: String, value: isize) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_int(CompareType::NEQ, field, value));
        self
    }
    pub fn neq_f64(&mut self, field: String, value: f64) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_f64(CompareType::NEQ, field, value));
        self
    }
    pub fn neq_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script(CompareType::NEQ, field));
        self
    }
    pub fn neq_raw_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script(CompareType::NEQ, field));
        self
    }
    pub fn neq_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_camcal(CompareType::NEQ, field));
        self
    }
    pub fn neq_raw_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script_camcal(CompareType::NEQ, field));
        self
    }
    pub fn neq_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_as(CompareType::NEQ, field, alias));
        self
    }
    pub fn neq_raw_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp.push(FilterField::cmp_raw_script_as(
            CompareType::NEQ,
            field,
            alias,
        ));
        self
    }
}

impl SimpleFilterInner {
    pub fn like_str(&mut self, field: String, value: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_str(CompareType::LK, field, value));
        self
    }
    pub fn like_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script(CompareType::LK, field));
        self
    }
    pub fn like_raw_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script(CompareType::LK, field));
        self
    }
    pub fn like_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_camcal(CompareType::LK, field));
        self
    }
    pub fn like_raw_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script_camcal(CompareType::LK, field));
        self
    }
    pub fn like_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_as(CompareType::LK, field, alias));
        self
    }
    pub fn like_raw_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp.push(FilterField::cmp_raw_script_as(
            CompareType::LK,
            field,
            alias,
        ));
        self
    }
}

impl SimpleFilterInner {
    pub fn gt_str(&mut self, field: String, value: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_str(CompareType::GT, field, value));
        self
    }
    pub fn gt_int(&mut self, field: String, value: isize) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_int(CompareType::GT, field, value));
        self
    }
    pub fn gt_f64(&mut self, field: String, value: f64) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_f64(CompareType::GT, field, value));
        self
    }
    pub fn gt_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script(CompareType::GT, field));
        self
    }
    pub fn gt_raw_script(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script(CompareType::GT, field));
        self
    }
    pub fn gt_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_camcal(CompareType::GT, field));
        self
    }
    pub fn gt_raw_script_camcal(&mut self, field: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_raw_script_camcal(CompareType::GT, field));
        self
    }
    pub fn gt_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp
            .push(FilterField::cmp_script_as(CompareType::GT, field, alias));
        self
    }
    pub fn gt_raw_script_as(&mut self, field: String, alias: String) -> &mut Self {
        self.cmp.push(FilterField::cmp_raw_script_as(
            CompareType::GT,
            field,
            alias,
        ));
        self
    }
}

#[derive(Clone)]
pub enum FilterType {
    AND,
    OR,
    Express,
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

#[derive(Clone)]
struct FilterField {
    cp: CompareType,
    field: CompareField,
    value: Value,
}

impl FilterField {
    pub fn cmp_str(cp: CompareType, field: String, value: String) -> FilterField {
        FilterField {
            cp,
            field: CompareField::new(field),
            value: Value::String(value),
        }
    }
    pub fn cmp_int(cp: CompareType, field: String, value: isize) -> FilterField {
        FilterField {
            cp,
            field: CompareField::new(field),
            value: Value::Number(Number::from(value)),
        }
    }
    pub fn cmp_f64(cp: CompareType, field: String, value: f64) -> FilterField {
        FilterField {
            cp,
            field: CompareField::new(field),
            value: Value::Number(Number::from_f64(value).unwrap()),
        }
    }

    pub fn cmp_script(cp: CompareType, field: String) -> FilterField {
        Self::cmp_script_as(cp, field.clone(), field)
    }
    pub fn cmp_raw_script(cp: CompareType, field: String) -> FilterField {
        Self::cmp_raw_script_as(cp, field.clone(), field)
    }

    pub fn cmp_script_camcal(cp: CompareType, field: String) -> FilterField {
        let camel_field = CaseStyle::from_snakecase(field.clone()).to_camelcase();
        Self::cmp_script_as(cp, field, camel_field)
    }
    pub fn cmp_raw_script_camcal(cp: CompareType, field: String) -> FilterField {
        let camel_field = CaseStyle::from_snakecase(field.clone()).to_camelcase();
        Self::cmp_raw_script_as(cp, field, camel_field)
    }

    pub fn cmp_script_as(cp: CompareType, field: String, alias: String) -> FilterField {
        let value = match cp {
            CompareType::LK | CompareType::NLK => {
                format!("'%#{{{}}}%'", alias)
            }
            CompareType::RLK | CompareType::NRLK => {
                format!("'%#{{{}}}'", alias)
            }
            CompareType::LLK | CompareType::NLLK => {
                format!("'#{{{}}}%'", alias)
            }
            _ => {
                format!("#{{{}}}", alias)
            }
        };
        FilterField {
            cp,
            field: CompareField::new(field),
            value: Value::String(value),
        }
    }
    pub fn cmp_raw_script_as(cp: CompareType, field: String, alias: String) -> FilterField {
        let value = match cp {
            CompareType::LK | CompareType::NLK => {
                format!("'%${{{}}}%'", alias)
            }
            CompareType::RLK | CompareType::NRLK => {
                format!("'%${{{}}}'", alias)
            }
            CompareType::LLK | CompareType::NLLK => {
                format!("'${{{}}}%'", alias)
            }
            _ => {
                format!("${{{}}}", alias)
            }
        };
        FilterField {
            cp,
            field: CompareField::new(field),
            value: Value::String(value),
        }
    }
}

#[derive(Clone)]
enum CompareField {
    Column(String),
    Func(FuncCompareFieldInner),
    Query(QueryCompareFiledInner),
}

impl CompareField {
    pub fn new(column: String) -> Self {
        CompareField::Column(column)
    }
}

#[derive(Clone)]
pub struct FuncCompareFieldInner {}

#[derive(Clone)]
pub struct QueryCompareFiledInner {}

#[derive(Clone)]
pub enum OrderType {
    Asc,
    Desc,
}

#[derive(Clone)]
pub struct OrderField {
    field: String,
    otype: OrderType,
}

impl OrderField {
    pub fn asc(field: String) -> Self {
        OrderField {
            field,
            otype: OrderType::Asc,
        }
    }
    pub fn desc(field: String) -> Self {
        OrderField {
            field,
            otype: OrderType::Desc,
        }
    }
    pub fn desc_vec(field: Vec<String>) -> Vec<Self> {
        vec![]
    }

    pub fn asc_vec(field: Vec<String>) -> Vec<Self> {
        vec![]
    }

    pub fn asc_filed(field: SelectField) -> Self {
        OrderField {
            field: field.to_field(),
            otype: OrderType::Asc,
        }
    }

    pub fn desc_filed(field: SelectField) -> Self {
        OrderField {
            field: field.to_field(),
            otype: OrderType::Desc,
        }
    }

    pub fn asc_filed_vec(field: Vec<SelectField>) -> Vec<Self> {
        vec![]
    }
    pub fn desc_filed_vec(field: Vec<SelectField>) -> Vec<Self> {
        vec![]
    }
}
