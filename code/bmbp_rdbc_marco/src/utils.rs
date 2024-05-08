use case_style::Token;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Field, Lit, LitStr, Meta, Token, Type, TypePath};

use crate::types::{ValidMeta, ValidRule, ValidRuleMethod};
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
                    let field_name = field.ident.clone().unwrap().to_string();
                    for attr_items in field.attrs.iter() {
                        if attr_items.path().is_ident("valid") {
                            let (insert_rule, update_rule) = parse_valid_item(
                                attr_items.meta.clone(),
                                &ValidRuleMethod::INSERT_UPDATE,
                            );
                            if !insert_rule.is_empty() {
                                let insert_valid = ValidMeta::new(field.clone(), insert_rule);
                                insert_valid_field.push(insert_valid);
                            }
                            if !update_rule.is_empty() {
                                let update_valid = ValidMeta::new(field.clone(), update_rule);
                                update_valid_field.push(update_valid);
                            }
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

fn parse_valid_item(
    attrs: Meta,
    valid_rule_method: &ValidRuleMethod,
) -> (Vec<ValidRule>, Vec<ValidRule>) {
    let mut insert_rule_vec = vec![];
    let mut update_rule_vec = vec![];
    if let Meta::List(list) = attrs {
        if let Ok(meta_list) = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        {
            for meta in meta_list.iter() {
                if meta.path().is_ident("insert") {
                    let (item_insert_vec, item_update_vec) =
                        parse_valid_item(meta.clone(), &ValidRuleMethod::INSERT);
                    insert_rule_vec.extend_from_slice(item_insert_vec.as_slice());
                    update_rule_vec.extend_from_slice(item_update_vec.as_slice());
                } else if meta.path().is_ident("update") {
                    let (item_insert_vec, item_update_vec) =
                        parse_valid_item(meta.clone(), &ValidRuleMethod::UPDATE);
                    insert_rule_vec.extend_from_slice(item_insert_vec.as_slice());
                    update_rule_vec.extend_from_slice(item_update_vec.as_slice());
                } else {
                    // 计算校验规则
                    let valid_rule = ValidRule::default();
                    if let Ok(require_list) = meta.require_list() {
                        if let Ok(r_me) = require_list
                            .parse_args_with(Punctuated::<Lit, Token![,]>::parse_terminated)
                        {
                            println!(
                                "{}参数=======>{}",
                                meta.path().get_ident().unwrap().to_string(),
                                r_me.len()
                            );
                            for r_me_item in r_me.iter() {
                                match r_me_item {
                                    Lit::Str(lit_str) => {
                                        println!("======item=>:{}", lit_str.value())
                                    }
                                    Lit::Int(lit_int) => {
                                        println!("======item=>:{}", lit_int.to_string())
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    match valid_rule_method {
                        ValidRuleMethod::INSERT_UPDATE => {
                            insert_rule_vec.push(valid_rule.clone());
                            update_rule_vec.push(valid_rule.clone());
                        }
                        ValidRuleMethod::INSERT => {
                            insert_rule_vec.push(valid_rule);
                        }
                        ValidRuleMethod::UPDATE => {
                            update_rule_vec.push(valid_rule);
                        }
                    }
                }
            }
        }
    }
    (insert_rule_vec, update_rule_vec)
}

/// 驼峰转下划线 大写
pub(crate) fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}
