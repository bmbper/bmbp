use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

use crate::util;

pub fn page(_: TokenStream, page_struct_token: TokenStream) -> TokenStream {
    let page_struct = parse_macro_input!(page_struct_token as DeriveInput);
    let page_field: Vec<(String, TokenStream2)> = util::build_page_field_token_stream();
    let struct_field: Vec<(String, Field)> = util::parse_struct_field(&page_struct);
    let struct_merge_filed: Vec<TokenStream2> =
        util::merge_field_token_stream_from_field(page_field, struct_field);
    let struct_atts = &page_struct.attrs;
    let struct_name_ident = &page_struct.ident;
    let final_struct = quote!(
        #(#struct_atts)*
        pub struct #struct_name_ident{
           #(#struct_merge_filed),*
        }
    );
    final_struct.into()
}
