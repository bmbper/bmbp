use crate::types::*;
impl SqlBuilder for SelectBuilder {
    fn build(&self) -> String {
        match self {
            SelectBuilder::String(s) => s.clone(),
            SelectBuilder::Dynamic(d) => d.build(),
        }
    }
}
impl SqlBuilder for DynamicSelectBuilder {
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
impl SqlBuilder for DynamicSelectBuilderType {
    fn build(&self) -> String {
        match self {
            DynamicSelectBuilderType::String(s) => s.clone(),
            DynamicSelectBuilderType::Dynamic(d) => d.build(),
            DynamicSelectBuilderType::TempTable(t) => t.build(),
        }
    }
}
impl SqlBuilder for TableBuilder {
    fn build(&self) -> String {
        match self {
            TableBuilder::String(s) => s.clone(),
            TableBuilder::Dynamic(d) => d.build(),
        }
    }
}
impl SqlBuilder for DynamicTableBuilder {
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

impl SqlBuilder for DynamicTableBuilderType {
    fn build(&self) -> String {
        match self {
            DynamicTableBuilderType::String(s) => s.clone(),
            DynamicTableBuilderType::Dynamic(d) => d.build(),
            DynamicTableBuilderType::TempTable(t) => t.build(),
        }
    }
}

impl SqlBuilder for QueryBuilder {
    fn build(&self) -> String {
        let mut sql = "SELECT ".to_string();
        sql += &self.select_.iter().map(|s| s.build()).collect::<Vec<_>>().join(", ");
        sql += &self.from_.iter().map(|t| t.build()).collect::<Vec<_>>().join(", ");
        if !self.join_.is_empty() {
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
impl SqlBuilder for JoinQueryBuilder {
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
impl SqlBuilder for JoinTableType {
    fn build(&self) -> String {
        match self {
            JoinTableType::Inner => "INNER JOIN".to_string(),
            JoinTableType::Left => "LEFT JOIN".to_string(),
            JoinTableType::Right => "RIGHT JOIN".to_string(),
            JoinTableType::Full => "FULL JOIN".to_string(),
        }
    }
}
impl SqlBuilder for QueryFilterBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &format!("{} (", self.typ.to_string());
        sql += &self.filters.iter().map(|f| f.build()).collect::<Vec<_>>().join(", ");
        sql += &")";
        sql
    }
}

impl SqlBuilder for QueryFilterType {
    fn build(&self) -> String {
        match self {
            QueryFilterType::And => "AND".to_string(),
            QueryFilterType::Or => "OR".to_string(),
        }
    }
}

impl SqlBuilder for QueryFilterItemBuilder {
    fn build(&self) -> String {
        match self {
            QueryFilterItemBuilder::Simple(s) => s.build(),
            QueryFilterItemBuilder::Nested(n) => n.build(),
        }
    }
}
impl SqlBuilder for QuerySimpleFilterItemBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &format!("{} ", self.filter_typ_.to_string());
        sql += &self.field_.build();
        sql += &format!(" {} ", self.op_.to_string());
        sql += &self.value_;
        sql
    }
}

impl SqlBuilder for FilterFieldBuilder {
    fn build(&self) -> String {
        match self {
            FilterFieldBuilder::String(s) => s.clone(),
            FilterFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}

impl SqlBuilder for DynamicFilterFieldBuilder {
    fn build(&self) -> String {
        match self {
            DynamicFilterFieldBuilder::String(s) => s.clone(),
            DynamicFilterFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}

impl SqlBuilder for FilterOperatorType {
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

impl SqlBuilder for OrderBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &self.field;
        sql += &format!(" {}", self.typ.to_string());
        sql
    }
}
impl SqlBuilder for OrderFieldBuilder {
    fn build(&self) -> String {
        match self {
            OrderFieldBuilder::String(s) => s.clone(),
            OrderFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}
impl SqlBuilder for DynamicOrderFieldBuilder {
    fn build(&self) -> String {
        match self {
            DynamicOrderFieldBuilder::String(s) => s.clone(),
            DynamicOrderFieldBuilder::Dynamic(d) => d.build(),
        }
    }
}
impl SqlBuilder for OrderType {
    fn build(&self) -> String {
        match self {
            OrderType::Asc => "ASC".to_string(),
            OrderType::Desc => "DESC".to_string(),
        }
    }
}

impl SqlBuilder for UpdateBuilder {
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
impl SqlBuilder for UpdateSetFieldBuilder {
    fn build(&self) -> String {
        let mut sql = "".to_string();
        sql += &self.field_.build();
        sql += &" = ";
        sql += &self.value_;
        sql
    }
}
impl SqlBuilder for DeleteBuilder {
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

impl SqlBuilder for InsertBuilder {
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
