use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::parse::Parse;
use syn::{Expr, Field, Meta, Token};

#[derive(Debug, Default, Clone)]
pub(crate) struct RdbcModelMeta {
    table_name: Option<String>,
    tree_prefix: Option<String>,
}

impl Parse for RdbcModelMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut table_name = None;
        let mut tree_prefix = None;
        if input.is_empty() {
            return Ok(RdbcModelMeta {
                table_name,
                tree_prefix,
            });
        }
        while !input.is_empty() {
            if let Ok(meta) = input.parse::<Meta>() {
                match meta {
                    Meta::NameValue(name_value) => {
                        if name_value.path.is_ident("table") {
                            table_name = Some(name_value.value.to_token_stream().to_string());
                        } else if name_value.path.is_ident("tree") {
                            tree_prefix = Some(name_value.value.to_token_stream().to_string());
                        }
                    }
                    Meta::Path(path) => {
                        let path_name = path.get_ident().unwrap().to_string();
                        if table_name.is_none() {
                            table_name = Some(path_name);
                        } else {
                            tree_prefix = Some(path_name)
                        }
                    }
                    Meta::List(_list) => {}
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(RdbcModelMeta {
            table_name,
            tree_prefix,
        })
    }
}

impl RdbcModelMeta {
    pub fn new(table_name: String, tree_prefix: String) -> Self {
        RdbcModelMeta {
            table_name: Some(table_name),
            tree_prefix: Some(tree_prefix),
        }
    }
    pub fn get_table_name(&self) -> Option<String> {
        self.table_name.clone()
    }
    pub fn get_tree_prefix(&self) -> Option<String> {
        self.tree_prefix.clone()
    }
    pub fn set_table_name(&mut self, table_name: String) {
        self.table_name = Some(table_name);
    }
    pub fn set_tree_prefix(&mut self, tree_prefix: String) {
        self.tree_prefix = Some(tree_prefix);
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
