use bmbp_app_common::BmbpBaseModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BmbpFileInfo {
    // 公共基础字段
    base: BmbpBaseModel,
    // 文件ID
    file_id: Option<String>,
    // 文件名称
    file_name: Option<String>,
    // 文件路径
    file_url: Option<String>,
    // 文件类型
    file_type: Option<String>,
    // 文件大小
    file_size: Option<usize>,
    // 所属应用
    app_code: Option<String>,
    // 所属模块
    module_code: Option<String>,
    // 所属功能
    func_code: Option<String>,
    // 所属属性
    field_name: Option<String>,
}
