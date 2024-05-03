use proc_macro::{TokenStream, TokenTree};

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Field, parse_macro_input};

use crate::types::ATTRS_RDBC_SKIP;
use crate::utils::{
    build_base_struct_model, camel_to_snake, get_struct_field, get_struct_field_name_map,
    has_rdbc_attr, is_struct_option_field,
};

pub(crate) fn rdbc_model(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    let base_model_token_stream = build_base_struct_model();
    let base_input_token = parse_macro_input!(base_model_token_stream as DeriveInput);
    let model_input_token = parse_macro_input!(model_token as DeriveInput);
    /// 表名称
    let struct_ident = &model_input_token.ident;
    let struct_table_name = build_struct_table_name(&meta_token, struct_ident);
    let struct_field_vec = build_struct_field_vec(&base_input_token, &model_input_token);

    let mut token_vec: Vec<TokenStream2> = vec![];

    // 构建结构体-增加默认字段
    let model_struct_token = build_struct_model_token(struct_ident, struct_field_vec.as_slice());
    // 构建结构体-实现属性方法
    let model_impl_get_set_token =
        build_struct_impl_get_set_token(struct_ident, struct_field_vec.as_slice());
    // 构建结构体-获取表名称、获取字段的方法
    let model_impl_orm_table_token = build_struct_impl_orm_table_token(
        struct_ident,
        &struct_table_name,
        struct_field_vec.as_slice(),
    );
    /// 构建结构体-增删 改查 脚本
    let struct_sql_script_token = build_struct_sql_script_token(&struct_ident);

    let model_dao_token = build_dao_token(&model_input_token, &struct_table_name);
    let model_service_token = build_service_token(&model_input_token, &struct_table_name);
    let model_curd_token = build_curd_token(&model_input_token, &struct_table_name);
    token_vec.push(model_struct_token);
    token_vec.push(model_impl_get_set_token);
    token_vec.push(model_impl_orm_table_token);
    token_vec.push(struct_sql_script_token);
    token_vec.push(model_dao_token);
    token_vec.push(model_service_token);
    token_vec.push(model_curd_token);
    let final_token = quote! {
       #(#token_vec)*
    };
    println!("最终输出{:?}", final_token.to_string());
    final_token.into()
}

fn build_struct_field_vec(
    base_struct_input: &DeriveInput,
    struct_input: &DeriveInput,
) -> Vec<Field> {
    let base_field = get_struct_field(base_struct_input);
    let mut model_field = get_struct_field(struct_input);
    let model_field_map = get_struct_field_name_map(&model_field);
    for mut field in base_field {
        let field_name = field.ident.as_mut().unwrap().to_string();
        if !model_field_map.contains_key(field_name.as_str()) {
            model_field.push(field);
        }
    }
    model_field
}

/// 获取表名
fn build_struct_table_name(meta_token: &TokenStream, struct_ident: &Ident) -> String {
    // 获取表名
    let mut model_table_name = get_table_name_by_meta(&meta_token);
    if model_table_name.is_empty() {
        model_table_name = struct_ident.to_string();
    }
    model_table_name = camel_to_snake(model_table_name);
    model_table_name
}

fn build_struct_model_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_model_fields = build_struct_field_token(&struct_fields);
    let token = quote! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_ident {
            #(#struct_model_fields,)*
        }
    };
    token
}
fn build_struct_field_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut field_vec = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if is_struct_option_field(field_type) {
            field_vec.push(quote! {
                 #field_ident: #field_type
            });
        } else {
            field_vec.push(quote! {
                 #field_ident: Option<#field_type>
            });
        }
    }
    field_vec
}

fn build_struct_impl_get_set_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_method_token = build_struct_impl_method_token_vec(struct_fields);
    let token = quote! {
        impl #struct_ident {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
        }
    };
    token
}

fn build_struct_impl_method_token_vec(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let set_method_name = format_ident!("set_{}", field_name);
        let get_method_name = format_ident!("get_{}", field_name);
        let get_mut_method_name = format_ident!("get_mut_{}", field_name);
        let field_type = &item.ty;
        let method_token = if is_struct_option_field(field_type) {
            quote! {
                pub fn #set_method_name(&mut self, value: #field_type) -> &mut Self {
                    self.#field_name = value;
                    self
                }
                pub fn #get_method_name(&self) -> &#field_type {
                    &self.#field_name
                }
                pub fn #get_mut_method_name(&mut self) -> &mut #field_type {
                    &mut self.#field_name
                }
            }
        } else {
            quote! {
                 pub fn #set_method_name(&mut self, value: Option<#field_type>) -> &mut Self {
                    self.#field_name = value;
                    self
                }
                pub fn #get_method_name(&self) -> &Option<#field_type> {
                    &self.#field_name
                }
                pub fn #get_mut_method_name(&mut self) ->&mut Option<#field_type> {
                    &mut self.#field_name
                }
            }
        };
        method_vec.push(method_token);
    }
    method_vec
}

fn build_struct_impl_orm_table_token(
    struct_ident: &Ident,
    struct_table_name: &String,
    struct_fields: &[Field],
) -> TokenStream2 {
    let mut struct_columns = vec![];
    for field in struct_fields {
        if has_rdbc_attr(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap();
        struct_columns.push(field_name.to_string());
    }
    let token = quote! {
        impl #struct_ident {
            pub fn get_table_name() -> String {
                return #struct_table_name.to_string();
            }
            pub fn get_table_primary_key() -> String {
                return "data_id".to_string();
            }
            pub fn get_table_columns() -> Vec<String> {
                return vec![
                    #(#struct_columns.to_string(),)*
                ];
            }
        }

    };
    token
}

fn build_struct_sql_script_token(struct_ident: &Ident) -> TokenStream2 {
    let script_token = quote! {
        impl #struct_ident {
            pub fn build_query_sql() -> Query {
                let mut query = Query::new();
                query.table(Self::get_table_name());
                query.select_vec(Self::get_table_columns());
                query
            }
            pub fn build_info_sql(data_id:&Option<String>) -> Query {
                let mut query = Query::new();
                query.table(Self::get_table_name());
                query.select_vec(Self::get_table_columns());
                query
            }
            pub fn build_remove_sql(data_id:&Option<String>) -> Delete {
                let mut delete = Delete::new();
                delete.table(Self::get_table_name());
                delete
            }
            pub fn build_enalbe_sql(data_id:&Option<String>) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update
            }
            pub fn build_disable_sql(data_id:&Option<String>) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update
            }
            pub fn build_update_status_sql(data_id:&Option<String>, status: i32 ) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update
            }
            pub fn build_update_flag_sql(data_id:&Option<String>, status: i32) -> Update {
               let mut update = Update::new();
                update.table(Self::get_table_name());
                update
            }
            pub fn build_insert_sql(&self) -> Insert {
                let insert = Insert::new();
                insert
            }
            pub fn build_update_sql(&self) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update
            }
        }
    };
    script_token
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
