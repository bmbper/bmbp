use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}
pub(crate) fn build_base_field_token() -> Vec<TokenStream2> {
    vec![
        quote! {
            data_id: Option<String>
        },
        quote! {
            data_level: Option<String>
        },
        quote! {
            data_status: Option<String>
        },
        quote! {
            data_flag: Option<String>
        },
        quote! {
            data_sort: Option<i32>
        },
        quote! {
            data_create_time: Option<String>
        },
        quote! {
            data_create_user: Option<String>
        },
        quote! {
            data_update_time: Option<String>
        },
        quote! {
            data_update_user: Option<String>
        },
        quote! {
            data_owner_org: Option<String>
        },
        quote! {
            data_sign: Option<String>
        },
    ]
}

pub(crate) fn get_field_name(field: &TokenStream2) -> String {
    field.to_string().split(":").collect::<Vec<_>>()[0].to_string()
}

pub(crate) fn build_field_name_map(feild_token_vec: &[TokenStream2]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for item in feild_token_vec {
        let field_name = get_field_name(item);
        map.insert(field_name.clone(), field_name.clone());
    }
    map
}
