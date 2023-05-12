use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

use crate::util;

pub fn model(_: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    let model_struct = parse_macro_input!(model_struct_token as DeriveInput);
    // 公共字段
    let base_field = util::build_model_field_token_stream();
    // 自定义字段
    let old_struct_field = util::parse_struct_field(&model_struct);
    // 合并字段
    let struct_merge_filed =
        util::merge_field_token_stream_from_field(base_field, old_struct_field);
    let struct_atts = &model_struct.attrs;
    let struct_name_ident: &Ident = &model_struct.ident;
    let final_struct = quote!(
        #(#struct_atts)*
        pub struct #struct_name_ident{
           #(#struct_merge_filed),*
        }
    );
    final_struct.into()
}
