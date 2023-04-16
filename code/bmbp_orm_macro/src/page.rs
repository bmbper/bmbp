use proc_macro::TokenStream as TokenStream0;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

use crate::util;

pub fn page(_: TokenStream0, page_struct_token: TokenStream0) -> TokenStream0 {
    // 处理Struct定义的属性
    let page_struct = parse_macro_input!(page_struct_token as DeriveInput);
    let page_field = vec![
        ("page_no".to_string(), "usize".to_string()),
        ("page_size".to_string(), "usize".to_string()),
    ];
    let (mut page_field_token, mut page_method_token) =
        util::build_token_by_field(page_field.as_slice());
    let (_, struct_field_token, struct_method_token) = util::parse_struct(&page_struct);
    page_field_token.extend_from_slice(struct_field_token.as_slice());
    page_method_token.extend_from_slice(struct_method_token.as_slice());
    let struct_name_ident: &Ident = &page_struct.ident;
    let new_struct_token = util::build(struct_name_ident, page_field_token, page_method_token);
    new_struct_token.into()
}
