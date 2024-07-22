use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{DeriveInput, Field};

/// parse_tree_meta 获取树型标记
pub fn parse_tree_meta(meta_token: TokenStream) -> String {
    meta_token.to_string()
}
pub fn parse_struct_fields(struct_input: &DeriveInput) -> Vec<Field> {
    let mut field_vec = vec![];
    match &struct_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    field_vec.push(field.clone())
                }
            }
            syn::Fields::Unnamed(_) => {}
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}

pub(crate) fn build_tree_field_name(tree_prefix: String) -> Vec<String> {
    vec![
        format!("{}_code", tree_prefix),
        format!("{}_parent_code", tree_prefix),
        format!("{}_name", tree_prefix),
        format!("{}_code_path", tree_prefix),
        format!("{}_name_path", tree_prefix),
        format!("{}_tree_grade", tree_prefix),
        format!("{}_leaf", tree_prefix),
        format!("{}_type", tree_prefix),
        format!("{}_children", tree_prefix),
    ]
}

pub(crate) fn build_tree_field(filed_names: &[String], struct_name: &Ident) -> Vec<Field> {
    let mut field_vec = vec![];
    for item in filed_names {
        let field_ident = format_ident!("{}", item);
    }
    field_vec
}
