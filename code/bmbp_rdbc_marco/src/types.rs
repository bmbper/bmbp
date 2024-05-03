use proc_macro2::TokenTree;

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
