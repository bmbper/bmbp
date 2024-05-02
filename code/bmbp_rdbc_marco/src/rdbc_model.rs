use proc_macro::{TokenStream, TokenTree};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, parse_macro_input, TypePath};
use syn::Type::Path;
use uuid::Uuid;

use crate::utils::{build_base_field_token, build_field_name_map, get_field_name};

pub(crate) fn rdbc_model(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    let mut model_table_name = get_table_name_by_meta(&meta_token);
    let model_input_token = parse_macro_input!(model_token as DeriveInput);
    let model_ident = &model_input_token.ident;
    if model_table_name.is_empty() {
        model_table_name = model_ident.to_string();
    }
    model_table_name = model_table_name.to_uppercase();
    println!("表名称：{}", model_table_name);
    let mut token_vec: Vec<TokenStream2> = vec![];
    let model_struct_token = build_struct_token(&model_input_token);
    let model_impl_token = build_method_token(&model_input_token);
    let model_trait_token = build_trait_token(&model_input_token);
    let model_script_token = build_script_token(&model_input_token, &model_table_name);
    let model_dao_token = build_dao_token(&model_input_token, &model_table_name);
    let model_service_token = build_service_token(&model_input_token, &model_table_name);
    let model_curd_token = build_curd_token(&model_input_token, &model_table_name);
    token_vec.push(model_struct_token);
    token_vec.push(model_impl_token);
    token_vec.push(model_trait_token);
    token_vec.push(model_script_token);
    token_vec.push(model_dao_token);
    token_vec.push(model_service_token);
    token_vec.push(model_curd_token);
    let final_token = quote! {
       #(#token_vec)*
    };
    println!("最终输出{:?}", final_token.to_string());
    final_token.into()
}

fn build_struct_token(input: &DeriveInput) -> TokenStream2 {
    let ident = &input.ident;
    let base_field_token: Vec<TokenStream2> = build_base_field_token();
    let mut struct_field_vec = build_field_token(input);
    let struct_field_map = build_field_name_map(struct_field_vec.as_slice());
    for item in base_field_token {
        if struct_field_map.contains_key(get_field_name(&item).as_str()) {
            continue;
        }
        struct_field_vec.push(item);
    }
    let token = quote! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #ident {
            #(#struct_field_vec,)*
        }
    };
    token
}

fn build_field_token(input: &DeriveInput) -> Vec<TokenStream2> {
    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                let mut field_vec = vec![];
                for field in fields_named.named.iter() {
                    let field_ident = &field.ident;
                    let field_type = &field.ty;
                    if let Path(TypePath { path, .. }) = field_type {
                        if path.segments.len() == 1 {
                            if path.segments[0].ident.to_string() == "Option" {
                                let field_token = quote! {
                                    #field_ident: #field_type
                                };
                                field_vec.push(field_token)
                            } else {
                                let field_token = quote! {
                                    #field_ident: Option<#field_type>
                                };
                                field_vec.push(field_token)
                            }
                        }
                    } else {
                        let field_token = quote! {
                                #field_ident: Option<#field_type>
                        };
                        field_vec.push(field_token)
                    }
                }
                field_vec
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut field_vec = vec![];
                for field in fields_unnamed.unnamed.iter() {
                    let field_type = &field.ty;
                    let field_name = format_ident!("field_{}", Uuid::new_v4().to_string());
                    let field_token = quote! {
                       #field_name : Option<#field_type>
                    };
                    field_vec.push(field_token)
                }
                field_vec
            }
            syn::Fields::Unit => {
                vec![]
            }
        },
        _ => {
            vec![]
        }
    }
}
fn build_method_token(input: &DeriveInput) -> TokenStream2 {
    let ident = &input.ident;
    let get_ste_token_vec = build_get_set_method_token_vec(input);
    let token = quote! {
        impl #ident {
              #(#get_ste_token_vec)*
        }
    };
    token
}

fn build_get_set_method_token_vec(input: &DeriveInput) -> Vec<TokenStream2> {
    vec![]
}

fn build_trait_token(input: &DeriveInput) -> TokenStream2 {
    let ident = &input.ident;
    let token = quote! {
        impl From<RdbcOrmRow> for #ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #ident::default();
                model
            }
        }
    };
    token
}

fn build_script_token(input: &DeriveInput, p1: &String) -> TokenStream2 {
    let ident = &input.ident;
    let script_ident = format_ident!("{}Script", ident);
    let token = quote! {
        pub struct #script_ident;
        impl #script_ident {

        }
    };
    token
}
fn build_dao_token(input: &DeriveInput, p1: &String) -> TokenStream2 {
    let ident = &input.ident;
    let script_ident = format_ident!("{}Dao", ident);
    let token = quote! {
        pub struct #script_ident;
        impl #script_ident {

        }
    };
    token
}
fn build_service_token(input: &DeriveInput, table_name: &String) -> TokenStream2 {
    let ident = &input.ident;
    let script_ident = format_ident!("{}Service", ident);
    let token = quote! {
        pub struct #script_ident;
        impl #script_ident {

        }
    };
    token
}
fn build_curd_token(input: &DeriveInput, table_name: &String) -> TokenStream2 {
    let ident = &input.ident;
    let script_ident = format_ident!("{}Curd", ident);
    let token = quote! {
        pub struct #script_ident;
        impl #script_ident {
        }
    };
    token
}

/// #[rdbc_model(TABLE)]
/// #[rdbc_model("TABLE")]
/// #[rdbc_model(table = TABLE)]
/// #[rdbc_model(table="TABLE")]
fn get_table_name_by_meta(meta_token: &TokenStream) -> String {
    let mut token_vec = vec![];
    for item in meta_token.clone().into_iter() {
        token_vec.push(item);
    }
    if token_vec.len() == 0 {
        return "".to_string();
    }
    if token_vec.len() == 1 {
        let item_slice = token_vec.get(0).unwrap();
        match item_slice {
            TokenTree::Ident(ident) => {
                return ident.to_string();
            }
            TokenTree::Literal(literal) => {
                return literal.to_string().replace("\"", "");
            }
            _ => {
                panic!(
                    "rdbc_model宏支持格式如下：{}\n,{}\n{}\n{}",
                    "#[rdbc_model(TABLE)]",
                    "#[rdbc_model(\"TABLE\")]",
                    "#[rdbc_model(table = TABLE)]",
                    "#[rdbc_model(table=\"TABLE\")]"
                );
            }
        }
    }
    if token_vec.len() == 2 {
        panic!(
            "rdbc_model宏支持格式如下：{}\n,{}\n{}\n{}",
            "#[rdbc_model(TABLE)]",
            "#[rdbc_model(\"TABLE\")]",
            "#[rdbc_model(table = TABLE)]",
            "#[rdbc_model(table=\"TABLE\")]"
        );
    }
    if token_vec.len() == 3 {
        let props = token_vec.get(0).unwrap();
        let punct = token_vec.get(1).unwrap();
        let value = token_vec.get(2).unwrap();
        if props.to_string() != "table" || punct.to_string() != "=" {
            panic!(
                "rdbc_model宏支持格式如下：{}\n,{}\n{}\n{}",
                "#[rdbc_model(TABLE)]",
                "#[rdbc_model(\"TABLE\")]",
                "#[rdbc_model(table = TABLE)]",
                "#[rdbc_model(table=\"TABLE\")]"
            );
        }
        return value.to_string().replace("\"", "");
    }
    panic!(
        "rdbc_model宏支持格式如下：{}\n,{}\n{}\n{}",
        "#[rdbc_model(TABLE)]",
        "#[rdbc_model(\"TABLE\")]",
        "#[rdbc_model(table = TABLE)]",
        "#[rdbc_model(table=\"TABLE\")]"
    );
}
