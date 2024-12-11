use crate::meta::TableTreeMeta;
use crate::util::{
    base_tree_table_field, build_impl_orm_row_token, build_impl_table_token, build_impl_tree_token,
    build_struct_token, build_table_name, merge_struct_fields, parse_struct_fields,
};
use bmbp_sql::RdbcColumnIdent;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Field};

pub(crate) fn generate_tree_table(
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
    let tree_prefix = rdbc_meta.get_tree().unwrap_or(&"".to_string()).to_string();
    let table_tree_field = base_tree_table_field(tree_prefix.clone(), struct_ident);
    struct_fields = merge_struct_fields(struct_fields, table_tree_field.as_slice());

    let struct_real_token = build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_fields.as_slice(),
    );
    let table_name = build_table_name(&rdbc_meta, struct_ident);
    let tree_children_field_name = format!("{}_children", tree_prefix.clone());
    let mut tree_struct_field = vec![];

    // impl Tree
    let impl_tree_token = build_impl_tree_token(struct_ident, tree_prefix.clone());

    for field in struct_fields.as_slice() {
        let field_name = field.ident.as_ref().unwrap().name();
        if field_name == tree_children_field_name.as_str() {
            continue;
        }
        tree_struct_field.push(field.clone());
    }
    let impl_table_token =
        build_impl_table_token(struct_ident, tree_struct_field.as_slice(), table_name);

    let impl_orm_row = build_impl_orm_row_token(struct_ident, tree_struct_field.as_slice());

    let table_struct_token = quote! {
        #struct_real_token
        #impl_table_token
        #impl_tree_token
        #impl_orm_row
    };
    table_struct_token.into()
}
