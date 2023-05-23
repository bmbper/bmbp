use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::{parse_macro_input, Field};

use crate::util;

pub fn bmbp_value(_: TokenStream, struct_token: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(struct_token as DeriveInput);
    // 类字段
    let struct_field: Vec<(String, Field)> = util::parse_struct_field(&derive_input);
    let struct_ident = &derive_input.ident;
    let struct_field_name: Vec<String> = struct_field
        .iter()
        .map(|(field_name, _)| field_name.to_string())
        .collect();
    let struct_field_name_ident: Vec<Ident> = struct_field
        .iter()
        .map(|(field_name, _)| format_ident!("{}", field_name.to_string()))
        .collect();
    let impl_struct_token = quote!(
        impl From<#struct_ident> for BmbpValue {
            fn from(value: #struct_ident) -> Self {
                let mut mp = BmbpHashMap::new();
                #(mp.insert(#struct_field_name.to_string(),BmbpValue::from(value.#struct_field_name_ident));)*
                BmbpValue::from(mp)
            }
        }
        impl From<&#struct_ident> for BmbpValue {
            fn from(value: &#struct_ident) -> Self {
                let mut mp = BmbpHashMap::new();
                #(mp.insert(#struct_field_name.to_string(),BmbpValue::from(&value.#struct_field_name_ident));)*
                BmbpValue::from(mp)
            }
        }
    );
    let final_struct = quote!(
        #derive_input
        #impl_struct_token
    );
    final_struct.into()
}
