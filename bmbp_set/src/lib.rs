use tracing;

/// 设置路由
pub fn set_route_scope() {
    tracing::info!("初始化设置模块路由")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
