use proc_macro::TokenStream as TokenStream0;
use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;

use crate::util;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Field, Ident, ItemStruct, Meta};

pub fn model(_: TokenStream0, model_struct_token: TokenStream0) -> TokenStream0 {
    // 处理Struct定义的属性
    let mut model_struct_field_token = vec![];
    let mut model_struct_method_token = vec![];
    // 处理Struct定义的属性
    let model_struct = parse_macro_input!(model_struct_token as DeriveInput);
    let (_, struct_field_token, struct_method_token) = util::parse_struct(&model_struct);
    model_struct_field_token.extend_from_slice(struct_field_token.as_slice());
    model_struct_method_token.extend_from_slice(struct_method_token.as_slice());
    let struct_name_ident: &Ident = &model_struct.ident;
    let new_struct_token = util::build(
        struct_name_ident,
        model_struct_field_token,
        model_struct_method_token,
    );
    new_struct_token.into()
}
