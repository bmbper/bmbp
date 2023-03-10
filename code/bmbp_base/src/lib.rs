/*
    存放公共的定义与全局变量
*/
pub mod cache;
mod curd;
pub mod func;
pub mod types;
pub mod vars;
mod vo;

pub use curd::CurdDao;
pub use curd::CurdPageService;
pub use curd::CurdService;
pub use curd::CurdTreePageService;
pub use curd::CurdTreeService;
