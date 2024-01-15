use crate::{DeleteBuilder, QueryBuilder, UpdateBuilder, InsertBuilder, TableBuilder, SelectBuilder, DynamicSelectBuilderType, DynamicSelectBuilder, FilterOperatorType, QueryFilterBuilder, QueryFilterType, QueryFilterItemBuilder, FilterFieldBuilder, QuerySimpleFilterItemBuilder, OrderBuilder, OrderType, SqlBuilder, UpdateSetFieldBuilder};

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            select_: vec![],
            from_: vec![],
            join_: vec![],
            filter_: None,
            group_: None,
            order_: None,
            having_: None,
            limit_: None,
            offset_: None,
        }
    }
    fn init_filter(&mut self) {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilterBuilder::and());
        }
    }

    fn build_filter(&mut self) {
        if let Some(filter) = self.filter_.as_mut() {
            if !filter.filters.is_empty() {
                let filter_str = filter.build();
                filter.filters.clear();
                filter.filters.push(QueryFilterItemBuilder::String(filter_str));
            }
        }
    }
    fn filter_mut(&mut self) -> &mut QueryFilterBuilder {
        self.init_filter();
        self.filter_.as_mut().unwrap()
    }
    pub fn or(&mut self) -> &mut Self {
        self.init_filter();
        // 将之前的字段渲染成一个新的filter
        self.build_filter();
        self.filter_mut().typ = QueryFilterType::Or;
        self
    }
    pub fn and(&mut self) -> &mut Self {
        self.init_filter();
        // 将之前的字段渲染成一个新的filter
        self.build_filter();
        self.filter_mut().typ = QueryFilterType::And;
        self
    }

    pub fn nest(&mut self) -> QueryFilterBuilder {
        QueryFilterBuilder::and()
    }
    pub fn nest_and(&mut self) -> QueryFilterBuilder {
        QueryFilterBuilder::and()
    }
    pub fn nest_or(&mut self) -> QueryFilterBuilder {
        QueryFilterBuilder::or()
    }

    pub fn add_filter(&mut self, filter: QueryFilterBuilder) -> &mut Self {
        self.init_filter();
        self.filter_mut().add_filter(filter);
        self
    }
    fn order_init(&mut self) -> &mut Self {
        if self.order_.is_none() {
            self.order_ = Some(vec![]);
        }
        self
    }
    fn order_mut(&mut self) -> &mut Vec<OrderBuilder> {
        self.order_init();
        self.order_.as_mut().unwrap()
    }
    fn group_init(&mut self) -> &mut Self {
        if self.group_.is_none() {
            self.group_ = Some(vec![]);
        }
        self
    }
    fn group_mut(&mut self) -> &mut Vec<SelectBuilder> {
        self.group_init();
        self.group_.as_mut().unwrap()
    }
    pub fn select(&mut self, field: &str) -> &mut Self {
        self.select_.push(SelectBuilder::String(field.to_string()));
        self
    }
    pub fn select_alias(&mut self, field: &str, alias: &str) -> &mut Self {
        self.select_.push(SelectBuilder::new_alise(field, alias));
        self
    }

    pub fn from(&mut self, table: &str) -> &mut Self {
        self.from_.push(TableBuilder::new(table));
        self
    }

    pub fn eq(&mut self, field: &str, value: &str) -> &mut Self {
        self.init_filter();
        self.filter_mut().eq(field, value);
        self
    }
    pub fn ne(&mut self, field: &str, value: &str) -> &mut Self {
        self.init_filter();
        self.filter_mut().ne(field, value);
        self
    }
    pub fn group(&mut self, field: &str) -> &mut Self {
        self.group_mut().push(SelectBuilder::new(field));
        self
    }

    pub fn order(&mut self, field: &str) -> &mut Self {
        self.order_init();
        self.order_mut().push(OrderBuilder::new(field));
        self
    }
    pub fn order_desc(&mut self, field: &str) -> &mut Self {
        self.order_init();
        self.order_mut().push(OrderBuilder::desc(field));
        self
    }
    pub fn order_asc(&mut self, field: &str) -> &mut Self {
        self.order_init();
        self.order_mut().push(OrderBuilder::asc(field));
        self
    }
}

impl SelectBuilder {
    pub fn new(field: &str) -> SelectBuilder {
        SelectBuilder::String(field.to_string())
    }
    pub fn new_alise(field: &str, alias: &str) -> SelectBuilder {
        let dynamic_field = DynamicSelectBuilder::new(field, alias);
        SelectBuilder::Dynamic(dynamic_field)
    }
}

impl DynamicSelectBuilder {
    pub fn new(filed: &str, alias: &str) -> DynamicSelectBuilder {
        DynamicSelectBuilder {
            table_alias_: None,
            field_: Box::new(DynamicSelectBuilderType::new(filed)),
            alias_: Some(alias.to_string()),
        }
    }
}

impl DynamicSelectBuilderType {
    pub fn new(field: &str) -> DynamicSelectBuilderType {
        DynamicSelectBuilderType::String(field.to_string())
    }
}

impl TableBuilder {
    pub fn new(table: &str) -> TableBuilder {
        TableBuilder::String(table.to_string())
    }
}

impl QueryFilterBuilder {
    pub fn new() -> Self {
        QueryFilterBuilder::and()
    }
    pub fn or() -> Self {
        QueryFilterBuilder {
            typ: QueryFilterType::Or,
            filters: vec![],
        }
    }
    pub fn and() -> Self {
        QueryFilterBuilder {
            typ: QueryFilterType::And,
            filters: vec![],
        }
    }

    pub fn eq(&mut self, field: &str, value: &str) -> &mut Self {
        self.filters.push(QueryFilterItemBuilder::new(field, value, FilterOperatorType::Eq));
        self
    }

    pub fn ne(&mut self, field: &str, value: &str) -> &mut Self {
        self.filters.push(QueryFilterItemBuilder::new(field, value, FilterOperatorType::Ne));
        self
    }
    pub fn add_filter(&mut self, filter: QueryFilterBuilder) -> &mut Self {
        self.filters.push(QueryFilterItemBuilder::Nested(filter));
        self
    }
}

impl QueryFilterItemBuilder {
    pub fn new(field: &str, value: &str, op: FilterOperatorType) -> QueryFilterItemBuilder {
        QueryFilterItemBuilder::Simple(QuerySimpleFilterItemBuilder::new(field, value, op))
    }
}

impl QuerySimpleFilterItemBuilder {
    pub fn new(field: &str, value: &str, op: FilterOperatorType) -> QuerySimpleFilterItemBuilder {
        QuerySimpleFilterItemBuilder {
            filter_typ_: QueryFilterType::And,
            field_: FilterFieldBuilder::String(field.to_string()),
            value_: value.to_string(),
            op_: op,
        }
    }
}

impl OrderBuilder {
    pub fn new(field: &str) -> OrderBuilder {
        OrderBuilder {
            field: field.to_string(),
            typ: OrderType::Asc,
        }
    }
    pub fn asc(field: &str) -> OrderBuilder {
        OrderBuilder {
            field: field.to_string(),
            typ: OrderType::Asc,
        }
    }
    pub fn desc(field: &str) -> OrderBuilder {
        OrderBuilder {
            field: field.to_string(),
            typ: OrderType::Desc,
        }
    }
}

impl UpdateBuilder {
    pub fn new() -> UpdateBuilder {
        UpdateBuilder {
            set_: vec![],
            from_: vec![],
            join_: vec![],
            filter_: None,
        }
    }
    fn init_filter(&mut self) {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilterBuilder::and());
        }
    }
    fn filter_mut(&mut self) -> &mut QueryFilterBuilder {
        self.init_filter();
        self.filter_.as_mut().unwrap()
    }
    pub fn from(&mut self, table: &str) -> &mut Self {
        self.from_.push(TableBuilder::new(table));
        self
    }

    pub fn set(&mut self, field: &str, value: &str) -> &mut Self {
        self.set_.push(UpdateSetFieldBuilder::new(field, value));
        self
    }

    pub fn eq(&mut self, field: &str, value: &str) -> &mut Self {
        self.init_filter();
        self.filter_mut().eq(field, value);
        self
    }
}

impl UpdateSetFieldBuilder {
    pub fn new(field: &str, value: &str) -> UpdateSetFieldBuilder {
        UpdateSetFieldBuilder {
            field_: FilterFieldBuilder::String(field.to_string()),
            value_: value.to_string(),
        }
    }
}

impl InsertBuilder {
    pub fn new() -> InsertBuilder {
        InsertBuilder {
            into_: None,
            fields_: None,
            values_: None,
            query_: None,
        }
    }

    fn fields_mut(&mut self) -> &mut Vec<String> {
        if self.fields_.is_none() {
            self.fields_ = Some(vec![]);
        }
        self.fields_.as_mut().unwrap()
    }
    fn values_mut(&mut self) -> &mut Vec<String> {
        if self.values_.is_none() {
            self.values_ = Some(vec![]);
        }
        self.values_.as_mut().unwrap()
    }
    pub fn into_table(&mut self, table: &str) -> &mut Self {
        self.into_ = Some(TableBuilder::new(table));
        self
    }
    pub fn field_value(&mut self, field: &str, value: &str) -> &mut Self {
        self.fields_mut().push(field.to_string());
        self.values_mut().push(value.to_string());
        self
    }
    pub fn fields(&mut self, fields: &[&str]) -> &mut Self {
        self.fields_ = Some(fields.iter().map(|f| f.to_string()).collect());
        self
    }
    pub fn values(&mut self, values: &[&str]) -> &mut Self {
        self.values_ = Some(values.iter().map(|f| f.to_string()).collect());
        self
    }
    pub fn query(&mut self, query: QueryBuilder) -> &mut Self {
        self.query_ = Some(query);
        self
    }
}

impl DeleteBuilder {
    pub fn new() -> DeleteBuilder {
        DeleteBuilder {
            from_: vec![],
            filter_: None,
        }
    }
    fn init_filter(&mut self) {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilterBuilder::and());
        }
    }
    fn filter_mut(&mut self) -> &mut QueryFilterBuilder {
        self.init_filter();
        self.filter_.as_mut().unwrap()
    }
    pub fn from(&mut self, table: &str) -> &mut Self {
        self.from_.push(TableBuilder::new(table));
        self
    }

    pub fn eq(&mut self, field: &str, value: &str) -> &mut Self {
        self.init_filter();
        self.filter_mut().eq(field, value);
        self
    }
}
