pub enum FormtType {}

/// ValidType 验证类型
pub enum ValidType {
    /// NULL值
    NotNone,
    /// 空值
    NotEmpty,
    /// 值大小
    Value(isize, isize),
    /// 长度
    Length(usize, usize),
    /// 格式校验
    Format(FormtType),
    /// 正则校验
    Regex(String),
}

/// ValidRule 校验规则 (校验类型,提示信息)
pub struct ValidRule(pub ValidType, pub String);
impl ValidRule {
    pub fn typ(&self) -> &ValidType {
        &self.0
    }
    pub fn msg(&self) -> &String {
        &self.1
    }
}

/// FieldValidRule 一个字段一个规则
pub struct FieldValidRule(pub String, pub ValidRule);
impl FieldValidRule {
    pub fn field(&self) -> &String {
        &self.0
    }
    pub fn rule(&self) -> &ValidRule {
        &self.1
    }
}

/// FieldValidRules 一个字段匹配多个规则
pub struct FieldValidRules(pub String, pub Vec<ValidRule>);

/// FieldsValidRule 多个字端使用一个规则
pub struct FieldsValidRule(pub Vec<String>, pub ValidRule);
impl FieldsValidRule {
    pub fn fields(&self) -> &[String] {
        self.0.as_slice()
    }

    pub fn rule(&self) -> &ValidRule {
        &self.1
    }
}
