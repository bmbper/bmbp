use std::collections::HashMap;
use bmbp_rdbc_sql::RdbcValue;

/// RdbcModel 表模型
pub trait RdbcModel{
    fn table_name() -> String;
    fn table_fields() -> Vec<String>;
    fn primary_key() -> String;
}

///RdbcPage 分页结果处理
pub struct RdbcPage<T>{
    page_size: usize,
    page_num: usize,
    total: usize,
    data: Option<Vec<T>>,
}
impl<T> RdbcPage<T> {
    pub fn new() -> Self {
        RdbcPage {
            page_size: 10,
            page_num: 1,
            total: 0,
            data: None,
        }
    }
    pub fn new_with_page(page_size: usize, page_num: usize) -> Self {
        RdbcPage {
            page_size,
            page_num,
            total: 0,
            data: None,
        }
    }
}
impl<T> RdbcPage<T> {
    pub fn page_num(&self) -> &usize {
        &self.page_num
    }
    pub fn page_size(&self) -> &usize {
        &self.page_size
    }
    pub fn total(&self) -> &usize {
        &self.total
    }
    pub fn data(&self) -> &Option<Vec<T>> {
        &self.data
    }
    pub fn set_page_num(&mut self, page_num: usize) -> &mut Self {
        self.page_num = page_num;
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_total(&mut self, total: usize) -> &mut Self {
        self.total = total;
        self
    }
    pub fn set_data(&mut self, data: Option<Vec<T>>) -> &mut Self {
        self.data = data;
        self
    }
}

pub struct RdbcRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}


