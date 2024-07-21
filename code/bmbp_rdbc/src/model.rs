use bmbp_rdbc_orm::RdbcOrm;
use bmbp_rdbc_sql::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};

pub trait RdbcActiveModel {
    async fn select_wrapper(&self) -> QueryWrapper;
    async fn insert_wrapper(&self) -> InsertWrapper;
    async fn insert_sensitive_wrapper(&self) -> InsertWrapper;
    async fn update_wrapper(&self) -> UpdateWrapper;
    async fn update_sensitive_wrapper(&self) -> UpdateWrapper;
    async fn delete_wrapper(&self) -> DeleteWrapper;
    async fn insert(&mut self, orm: &RdbcOrm) -> usize;
    async fn update(&mut self, orm: &RdbcOrm) -> usize;
    async fn delete(&mut self, orm: &RdbcOrm) -> usize;
    async fn get_one(orm: &RdbcOrm) -> Option<Self>;
    async fn select_one(orm: &RdbcOrm) -> Option<Self>;
    async fn select_list(orm: &RdbcOrm) -> Option<Self>;
    async fn select_all(orm: &RdbcOrm) -> Option<Self>;
    async fn select_page(orm: &RdbcOrm) -> Option<Self>;
}
