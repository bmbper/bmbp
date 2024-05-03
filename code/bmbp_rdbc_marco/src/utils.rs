use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Meta, Type, TypePath};
use syn::Type::Path;
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
pub(crate) fn get_struct_field_name_map(field: &[Field]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for item in field {
        map.insert(item.ident.clone().unwrap().to_string(), "".to_string());
    }
    map
}

/// 判断是否是OptionField
pub(crate) fn is_struct_option_field(field_type: &Type) -> bool {
    if let Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 {
            if path.segments[0].ident.to_string() == "Option" {
                return true;
            }
        }
    }
    false
}

pub(crate) fn has_rdbc_attr(field: &Field, attr: &str) -> bool {
    for attr_item in field.attrs.iter() {
        if let Ok(Meta::Path(path)) = attr_item.parse_meta() {
            if path.is_ident(attr) {
                return true;
            }
        }
    }
    false
}

/// 驼峰转下划线 大写
pub(crate) fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}
