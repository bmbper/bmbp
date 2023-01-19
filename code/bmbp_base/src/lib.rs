/*
    存放公共的定义与全局变量
*/
pub mod cache;
mod curd;
pub mod func;
pub mod types;
pub mod vars;

pub use curd::CurdDao;
pub use curd::CurdPageService;
pub use curd::CurdService;
pub use curd::CurdTreeService;
