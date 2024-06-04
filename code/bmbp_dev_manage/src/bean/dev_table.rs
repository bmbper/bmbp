/// BmbpDevDataSource 数据源管理
#[allow(dead_code)]
pub struct BmbpDevDataSource {
    ds_name: Option<String>,
    ds_meta: Option<String>,
}

/// BmbpDevTable 存储表结构描述
/// ```
/// ```
#[allow(dead_code)]
pub struct BmbpDevTable {
    table_name: Option<String>,
    table_meta: Option<String>,
}
/// BmbpDevHttp 存储接口描述
#[allow(dead_code)]
pub struct BmbpDevHttp {
    http_name: Option<String>,
    http_meta: Option<String>,
}
/// BmbpDevLogic 逻辑描述，HTTP的一部分
#[allow(dead_code)]
pub struct BmbpDevLogic {
    logic_name: Option<String>,
    logic_meta: Option<String>,
}

/// BmbpDevBpmn 流程描述
pub struct BmbpDevBpmn {
    bpmn_name: Option<String>,
    bpmn_meta: Option<String>,
}
/// BmbpDevProcess 数据处理
pub struct BmbpDevProcess {
    process_name: Option<String>,
    process_meta: Option<String>,
}
