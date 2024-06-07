/// DataSourceMetaVo 数据源配置信息
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DataSourceMetaVo {
    host: Option<String>,
    port: Option<String>,
    catalog: Option<String>,
    schema: Option<String>,
    username: Option<String>,
    password: Option<String>,
    url: Option<String>,
    db_type: Option<String>,
}
