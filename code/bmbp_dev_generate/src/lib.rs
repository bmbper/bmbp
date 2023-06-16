use log::info;

pub mod api;

/// init 模块初始化方法
pub fn init() {}

/// route_scope 构建配置中心模块的URL路由
pub fn route_scope() {
    info!("===> 初始化配置中心路由")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
