use crate::meta::TableTreeMeta;
use crate::util::{
    base_table_field, build_impl_orm_row_token, build_impl_table_token, build_struct_token,
    build_table_name, merge_struct_fields, parse_struct_fields,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
pub(crate) fn generate_table_bean(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let rdbc_meta = parse_macro_input!(meta_token as TableTreeMeta);
    let temp_struct_token = struct_token.clone();
    let struct_input = parse_macro_input!(temp_struct_token as syn::DeriveInput);
    let struct_ident = &struct_input.ident;
    let struct_generics = &struct_input.generics;
    let struct_attrs = &struct_input.attrs.as_slice();
    // 字段
    let mut struct_fields = parse_struct_fields(&struct_input);
    let struct_base_fields = base_table_field();
    struct_fields = merge_struct_fields(struct_fields, struct_base_fields.as_slice());
    let struct_real_token = build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_fields.as_slice(),
    );
    let table_name = build_table_name(&rdbc_meta, struct_ident);
    let impl_table_token =
        build_impl_table_token(struct_ident, struct_fields.as_slice(), table_name);

    let impl_orm_row = build_impl_orm_row_token(struct_ident, struct_fields.as_slice());

    let real_token = quote! {
        #struct_real_token
        #impl_table_token
        #impl_orm_row
    };
    real_token.into()
}
