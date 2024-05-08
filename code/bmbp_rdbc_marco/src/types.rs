use proc_macro2::TokenTree;
use syn::Field;

#[derive(Debug)]
pub(crate) struct RdbcModelMeta {
    table_name: String,
    tree_prefix: String,
}
impl RdbcModelMeta {
    pub fn new(table_name: String, tree_prefix: String) -> Self {
        RdbcModelMeta {
            table_name,
            tree_prefix,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MetaToken {
    pub(crate) props_token: Option<TokenTree>,
    pub(crate) punct_token: Option<TokenTree>,
    pub(crate) value_token: Option<TokenTree>,
}
impl MetaToken {
    pub fn new() -> Self {
        MetaToken {
            props_token: None,
            punct_token: None,
            value_token: None,
        }
    }
}

pub(crate) const ATTRS_RDBC_SKIP: &str = "rdbc_skip";
pub(crate) const ATTRS_QUERY: &str = "query";

pub struct ValidMeta {
    field: Field,
    valid: Vec<ValidRule>,
}

impl ValidMeta {
    pub fn new(field: Field, rule: Vec<ValidRule>) -> Self {
        ValidMeta { field, valid: rule }
    }
}
pub enum ValidRuleMethod {
    INSERT,
    UPDATE,
    INSERT_UPDATE,
}
#[derive(Debug, Default, Clone)]
pub struct ValidRule {
    typ: ValidRuleType,
    value: ValidRuleValue,
    msg: String,
}
#[derive(Debug, Clone)]
pub enum ValidRuleType {
    NotNull,
    Unique,
    Min,
    MinLength,
    Max,
    MaxLength,
    None,
}
impl Default for ValidRuleType {
    fn default() -> Self {
        ValidRuleType::None
    }
}
#[derive(Debug, Clone)]
pub enum ValidRuleValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    None,
}
impl Default for ValidRuleValue {
    fn default() -> Self {
        ValidRuleValue::None
    }
}
