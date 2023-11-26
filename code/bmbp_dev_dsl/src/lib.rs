// 开发工具规范定义部分
// 定义开发工具所支撑的数据库模块、接口模块、逻辑模块、界面模块
// 数据库模块：
//      定义数据库相关描述，用于生成DDL、DML、DQL
// 接口模块：
//      定义接口相关描述，用于处理HTTP请求
// 逻辑模块：
//      用于定义接口与数据库交互过程程中业务逻辑
// 流程模块：
//      用于组织逻辑模块，并生成执行流程图
// 业务模块：
//      定义业务相关描述，用于生成业务代码
// 界面模块：
//      定义界面相关描述，用于生成界面展示

mod http;
mod data;
mod flow;
mod logic;

pub use data::*;
