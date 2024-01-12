pub enum Wrapper {
    Insert(InsertWrapper),
    Query(QueryWrapper),
    UPDATE(UpdateWrapper),
    DELETE(DeleteWrapper),
}
pub struct InsertWrapper {}

pub struct UpdateWrapper {}

pub struct QueryWrapper {}

pub struct DeleteWrapper {}