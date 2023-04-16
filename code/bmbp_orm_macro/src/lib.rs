use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Field, Ident, ItemStruct, Meta};

mod model;
mod orm;
mod page;
mod util;
mod validator;

/// #[orm] 给Struct增加以下方法：
///      get_,get_mut_,set_
///      query_sql,insert_sql,update_sql,delete_sql
///      query_camel_sql,insert_camel_sql,update_camel_sql
///        
#[proc_macro_attribute]
pub fn orm(orm_meta_token: TokenStream, orm_struct_token: TokenStream) -> TokenStream {
    orm::orm(orm_meta_token, orm_struct_token)
}

/// #[model] 给Struct增加 get_,get_mut_,set_方法
#[proc_macro_attribute]
pub fn model(model_meta_token: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    model::model(model_meta_token, model_struct_token)
}

/// #[page] 给Struct增加 page_no,page_size,get_mut_,set_方法
#[proc_macro_attribute]
pub fn page(page_meta_token: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    page::page(page_meta_token, model_struct_token)
}

/// #[validator] 增加规则校验
#[proc_macro_attribute]
pub fn validator(model_meta_token: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    validator::validator(model_meta_token, model_struct_token)
}
