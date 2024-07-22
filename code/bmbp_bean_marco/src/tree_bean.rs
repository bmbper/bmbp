use crate::util;
use crate::util::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_tree_bean(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    let struct_input_token = parse_macro_input!(struct_token as DeriveInput);
    let struct_ident = struct_input_token.ident;
    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token);
    // 拼接树型字段
    let mut tree_field_name = util::build_tree_field_name(tree_prefix);
    let mut tree_field = util::build_tree_field(tree_field_name.as_slice(), &struct_ident);
    let struct_fields = parse_struct_fields(&struct_input_token);
    for field in struct_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if !tree_field_name.contains(&field_name) {
            tree_field.push(field);
        }
    }

    let real_struct = quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #struct_ident {
            #(#tree_field),*
        }
    };
    real_struct.into()
}
