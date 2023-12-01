use crate::types::{DeleteBuilder, InsertBuilder, QueryBuilder, SqlBuilder, UpdateBuilder};
pub enum SQL {
    Query(QueryBuilder),
    UPDATE(UpdateBuilder),
    DELETE(DeleteBuilder),
    INSERT(InsertBuilder),
}
impl SqlBuilder for SQL {
    fn build(&self) -> String {
        match self {
            SQL::Query(q) => q.build(),
            SQL::UPDATE(u) => u.build(),
            SQL::DELETE(d) => d.build(),
            SQL::INSERT(i) => i.build(),
        }
    }
}
