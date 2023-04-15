#[allow(dead_code)]
pub enum FormatRule {
    // 手机号
    PHONE,
    // 电子邮件
    EMAIL,
    // 身份证号
    CardId,
    // 网卡地址
    MAC,
    // 网络地址
    IP,
    // URL
    URL,
    // 日期格式
    DATE(String),
    // 字符
    CHAR,
    // 数字
    NUMBER,
    // 字符串
    CHARACTER,
    // 汉字
    CHINESE,
    // 全英文
    EN,
}
