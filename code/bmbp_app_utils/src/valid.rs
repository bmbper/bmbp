use bmbp_app_common::{
    BmbpError, BmbpHashMap, BmbpValue, FieldValidRule, FieldValidRules, FieldsValidRule, FormtType,
    ValidRule,
};

///valid_field_rule 单属性 单规则校验
pub fn valid_field_rule(_params: &BmbpHashMap, _rule: &FieldValidRule) -> Option<BmbpError> {
    None
}
/// valid_fields_rule 多属性单规则校验
pub fn valid_fields_rule(params: &BmbpHashMap, rule: &FieldsValidRule) -> Option<BmbpError> {
    let fields = rule.fields();
    let rule = rule.rule();
    for field in fields {
        let field_op = valid_field_value(params, field, rule);
        if field_op.is_some() {
            return field_op;
        }
    }
    None
}
/// valid_field_rules 单属性多规则校验
pub fn valid_field_rules(_params: &BmbpHashMap, _rule: &FieldValidRules) -> Option<BmbpError> {
    None
}

fn valid_field_value(params: &BmbpHashMap, filed: &String, rule: &ValidRule) -> Option<BmbpError> {
    let rule_type = rule.typ();
    let field_value = params.get(filed);
    let valid_msg = rule.msg();
    match rule_type {
        bmbp_app_common::ValidType::NotNone => valid_value_not_none(field_value, valid_msg),
        bmbp_app_common::ValidType::NotEmpty => valid_value_not_empty(field_value, valid_msg),
        bmbp_app_common::ValidType::Value(min, max) => {
            valid_value_size(field_value, valid_msg, min, max)
        }
        bmbp_app_common::ValidType::Length(min, max) => {
            valid_value_length(field_value, valid_msg, min, max)
        }
        bmbp_app_common::ValidType::Format(format) => {
            valid_value_format(field_value, valid_msg, format)
        }
        bmbp_app_common::ValidType::Regex(regex) => {
            valid_value_regex(field_value, valid_msg, regex)
        }
    }
}

fn valid_value_not_none(value: Option<&BmbpValue>, msg: &String) -> Option<BmbpError> {
    if value.is_none() {
        return Some(BmbpError::valid(msg));
    }
    if value.unwrap().is_null() {
        return Some(BmbpError::valid(msg));
    }
    None
}

fn valid_value_not_empty(value: Option<&BmbpValue>, msg: &String) -> Option<BmbpError> {
    if value.is_none() {
        return Some(BmbpError::valid(msg));
    }
    let raw_value = value.unwrap();
    if raw_value.is_null() {
        return Some(BmbpError::valid(msg));
    }
    if raw_value.is_string() && raw_value.raw_string().unwrap().is_empty() {
        return Some(BmbpError::valid(msg));
    }
    None
}
#[allow(dead_code)]
fn valid_value_size(
    value: Option<&BmbpValue>,
    msg: &String,
    min: &isize,
    max: &isize,
) -> Option<BmbpError> {
    None
}
#[allow(dead_code)]
fn valid_value_length(
    value: Option<&BmbpValue>,
    msg: &String,
    min: &usize,
    max: &usize,
) -> Option<BmbpError> {
    None
}
#[allow(dead_code)]
fn valid_value_format(
    value: Option<&BmbpValue>,
    msg: &String,
    format: &FormtType,
) -> Option<BmbpError> {
    None
}
fn valid_value_regex(value: Option<&BmbpValue>, msg: &String, regex: &String) -> Option<BmbpError> {
    None
}
