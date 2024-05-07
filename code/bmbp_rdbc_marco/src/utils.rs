use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::Expr::Let;
use syn::{Attribute, DeriveInput, Field, Fields, Meta, Token, Type, TypePath};

use crate::types::ValidMeta;
use uuid::Uuid;

/// 构建基础模型
pub(crate) fn build_base_struct_model() -> TokenStream {
    let base_model = quote! {
         pub struct BmbpOrmBaseModel {
             data_id: Option<String>,
             data_level: Option<String>,
             data_flag: Option<String>,
             data_status: Option<String>,
             data_sort: Option<i32>,
             data_create_time: Option<String>,
             data_create_user: Option<String>,
             data_update_time: Option<String>,
             data_update_user: Option<String>,
             data_owner_org: Option<String>,
             data_sign: Option<String>,
         }
    };
    base_model.into()
}
/// 获取结构体字段
pub(crate) fn get_struct_field(derive_input: &DeriveInput) -> Vec<Field> {
    let mut field_vec = vec![];
    match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    field_vec.push(field.clone())
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                for field in fields_unnamed.unnamed.iter() {
                    let mut name_field = field.clone();
                    name_field.ident = Some(format_ident!("field_{}", Uuid::new_v4().to_string()));
                    field_vec.push(name_field)
                }
            }
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}
/// 获取结构体字段
pub(crate) fn get_struct_field_by_attrs(derive_input: &DeriveInput, attrs: &str) -> Vec<Field> {
    let mut field_vec = vec![];
    match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    if has_rdbc_attr(field, attrs) {
                        field_vec.push(field.clone())
                    }
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                for field in fields_unnamed.unnamed.iter() {
                    let mut name_field = field.clone();
                    name_field.ident = Some(format_ident!("field_{}", Uuid::new_v4().to_string()));
                    if has_rdbc_attr(field, attrs) {
                        field_vec.push(name_field)
                    }
                }
            }
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}

/// 获取结构体字段
pub(crate) fn get_struct_field_name_map(field: &[Field]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for item in field {
        map.insert(item.ident.clone().unwrap().to_string(), "".to_string());
    }
    map
}

/// 判断是否是OptionField
pub(crate) fn is_struct_option_field(field_type: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 {
            if path.segments[0].ident.to_string() == "Option" {
                return true;
            }
        }
    }
    false
}

pub(crate) fn has_rdbc_attr(field: &Field, const_attr: &str) -> bool {
    for attr_item in field.attrs.iter() {
        return attr_item.path().is_ident(const_attr);
    }
    false
}

pub(crate) fn get_query_type(field: &Field) -> String {
    let mut field_type = "".to_string();
    for attr_item in field.attrs.iter() {
        if attr_item.path().is_ident("query") {
            let _ = attr_item.parse_nested_meta(|meta| {
                return if meta.path.is_ident("eq") {
                    field_type = "eq".to_string();
                    Ok(())
                } else if meta.path.is_ident("ne") {
                    field_type = "ne".to_string();
                    Ok(())
                } else if meta.path.is_ident("like") {
                    field_type = "like".to_string();
                    Ok(())
                } else if meta.path.is_ident("like_left") {
                    field_type = "like_left".to_string();
                    Ok(())
                } else if meta.path.is_ident("like_right") {
                    field_type = "like_right".to_string();
                    Ok(())
                } else {
                    Ok(())
                };
            });
        }
    }
    field_type
}

pub(crate) fn get_valid_field(derive_input: &DeriveInput) -> (Vec<ValidMeta>, Vec<ValidMeta>) {
    let mut insert_valid_field = vec![];
    let mut update_valid_field = vec![];
    match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    for attr_items in field.attrs.iter() {
                        if attr_items.path().is_ident("valid") {
                            println!("=====>解析TOKEN");
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }

    (insert_valid_field, update_valid_field)
}

/// 驼峰转下划线 大写
pub(crate) fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}
