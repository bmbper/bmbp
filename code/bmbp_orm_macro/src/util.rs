use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Field};

pub fn parse_struct_field(input_struct: &DeriveInput) -> Vec<(String, Field)> {
    let mut field_vec = vec![];
    let struct_data = &input_struct.data;
    match struct_data {
        Data::Struct(orm_struct_data) => {
            let orm_struct_fields = &orm_struct_data.fields;
            for struct_field in orm_struct_fields {
                field_vec.push((
                    struct_field.ident.as_ref().unwrap().to_string(),
                    struct_field.clone(),
                ))
            }
        }
        _ => {}
    }
    field_vec
}

#[allow(dead_code)]
pub fn parse_struct_field_name(input_struct: &DeriveInput) -> Vec<String> {
    let mut field_vec = vec![];
    let struct_data = &input_struct.data;
    match struct_data {
        Data::Struct(orm_struct_data) => {
            let orm_struct_fields = &orm_struct_data.fields;
            for struct_field in orm_struct_fields {
                let struct_field_name = &struct_field.ident.as_ref().unwrap().to_string();
                field_vec.push(struct_field_name.clone())
            }
        }
        _ => {}
    }
    field_vec
}

pub fn build_tree_field_token_stream(
    tree_prefix: String,
    tree_struct_name: String,
) -> Vec<(String, TokenStream)> {
    let tree_field_vec = vec![
        (format!("{}_{}", tree_prefix, "id"), "String".to_string()),
        (
            format!("{}_{}", tree_prefix, "id_path"),
            "String".to_string(),
        ),
        (format!("{}_{}", tree_prefix, "title"), "String".to_string()),
        (
            format!("{}_{}", tree_prefix, "title_path"),
            "String".to_string(),
        ),
        (
            format!("{}_{}", tree_prefix, "parent_id"),
            "String".to_string(),
        ),
        (
            format!("{}_{}", tree_prefix, "data_id"),
            "String".to_string(),
        ),
    ];
    let mut field_vec = build_field_token_stream(tree_field_vec.as_slice());
    let child_field_id = format_ident!("{}_{}", tree_prefix, "children");
    let struct_ident = format_ident!("{}", tree_struct_name);
    field_vec.push((
        child_field_id.to_string(),
        quote!(#child_field_id:Vec<#struct_ident>),
    ));
    field_vec
}

pub fn build_model_field_token_stream() -> Vec<(String, TokenStream)> {
    let field_vec = vec![
        ("r_id".to_string(), "String".to_string()),
        ("r_level".to_string(), "String".to_string()),
        ("r_flag".to_string(), "String".to_string()),
        ("r_create_time".to_string(), "String".to_string()),
        ("r_create_user".to_string(), "String".to_string()),
        ("r_update_time".to_string(), "String".to_string()),
        ("r_update_user".to_string(), "String".to_string()),
        ("r_owner_org".to_string(), "String".to_string()),
        ("r_owner_user".to_string(), "String".to_string()),
        ("r_sign".to_string(), "String".to_string()),
    ];
    build_field_token_stream(field_vec.as_slice())
}

pub fn build_page_field_token_stream() -> Vec<(String, TokenStream)> {
    let field_vec = vec![
        ("page_no".to_string(), "usize".to_string()),
        ("page_size".to_string(), "usize".to_string()),
        ("total".to_string(), "usize".to_string()),
    ];
    build_field_token_stream(field_vec.as_slice())
}

fn build_field_token_stream(field_slice: &[(String, String)]) -> Vec<(String, TokenStream)> {
    let mut tree_field_vec = vec![];
    for (name, type_) in field_slice {
        let name_ident = format_ident!("{}", name.to_string().replace("\"", ""));
        let type_ident = format_ident!("{}", type_.to_string().replace("\"", ""));
        tree_field_vec.push((name.to_string(), quote!(#name_ident: #type_ident)));
    }
    tree_field_vec
}

#[allow(dead_code)]
pub fn merge_field_token_stream(
    tree_field: Vec<(String, TokenStream)>,
    struct_field: Vec<(String, TokenStream)>,
) -> Vec<TokenStream> {
    let mut merge_field = vec![];
    let mut field_key_map = HashMap::new();
    for (key, item) in tree_field {
        field_key_map.insert(key, "");
        merge_field.push(item);
    }
    for (key, item) in struct_field {
        if field_key_map.contains_key(key.as_str()) {
            continue;
        }
        field_key_map.insert(key, "");
        merge_field.push(item);
    }
    merge_field
}

pub fn merge_field_token_stream_from_field(
    tree_field: Vec<(String, TokenStream)>,
    struct_field: Vec<(String, Field)>,
) -> Vec<TokenStream> {
    let mut merge_field = vec![];
    let mut field_key_map = HashMap::new();
    for (key, item) in tree_field {
        field_key_map.insert(key, "");
        merge_field.push(item);
    }
    for (key, item) in struct_field {
        if field_key_map.contains_key(key.as_str()) {
            continue;
        }
        field_key_map.insert(key, "");
        merge_field.push(item.into_token_stream());
    }
    merge_field
}
