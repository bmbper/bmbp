use bmbp_marco_bean::{rdbc_tree_bean, tree_bean};
use serde::Deserialize;
use serde::Serialize;

#[rdbc_tree_bean(vars)]
pub struct BmbpVars {
    value: String,
}
