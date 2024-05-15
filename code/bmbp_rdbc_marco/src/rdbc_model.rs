use proc_macro::{TokenStream, TokenTree};

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use salvo::Router;
use syn::parse::Parser;
use syn::{meta, parse, parse_macro_input, parse_quote, Attribute, DeriveInput, Field};

use crate::consts::*;
use crate::types::{RdbcModelMeta, ATTRS_QUERY, ATTRS_RDBC_SKIP};
use crate::utils::{
    camel_to_snake, get_base_model, get_base_tree_model, get_query_type, get_struct_field,
    get_struct_field_by_attrs, get_struct_field_name_map, get_valid_field, has_rdbc_attr,
    is_struct_option_field,
};

pub(crate) fn rdbc_model(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    // 基础模型
    let base_model_token = get_base_model();
    let base_input = parse_macro_input!(base_model_token as DeriveInput);
    let base_tree_model_token = get_base_tree_model();
    let base_tree_input = parse_macro_input!(base_tree_model_token as DeriveInput);
    let model_input_token = parse_macro_input!(model_token as DeriveInput);

    // 结构体名称
    let struct_ident = &model_input_token.ident;

    // 表名
    let mut mode_meta: RdbcModelMeta = parse_macro_input!(meta_token as RdbcModelMeta);
    if mode_meta.get_table_name().is_none() || mode_meta.get_table_name().unwrap().is_empty() {
        mode_meta.set_table_name(camel_to_snake(struct_ident.to_string()));
    } else {
        mode_meta.set_table_name(camel_to_snake(mode_meta.get_table_name().unwrap()));
    }
    // 树名称
    let struct_tree_prefix = mode_meta.get_tree_prefix();
    let struct_table_name = mode_meta.get_table_name().unwrap();

    let struct_field_vec = build_struct_field_vec(
        &base_input,
        &model_input_token,
        &base_tree_input,
        &struct_tree_prefix,
    );

    let mut model_struct_macro_token: Vec<TokenStream2> = vec![];
    //构建查询结构体
    // 构建查询结构体-属性
    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);
    let struct_query_field_vec = get_field_by_attrs(struct_field_vec.as_slice(), ATTRS_QUERY);
    /// 查询结构体
    let model_struct_query_token =
        build_struct_query_model_token(&struct_query_ident, struct_query_field_vec.as_slice());

    /// 查询结构体方法
    let mode_struct_query_impl_token =
        build_struct_query_impl_token(&struct_query_ident, struct_query_field_vec.as_slice());

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
    let model_impl_sql_token =
        build_struct_impl_script_token(&struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-RdbcOrmRow的转换
    let model_from_rdbc_orm_row_token =
        build_struct_from_rdbc_orm_row_token(struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-orm 操作数据库
    let model_orm_token = build_struct_orm_token(&struct_ident);

    /// 构建结构体-orm 增删改查方法
    let model_curd_token = build_struct_curd_token(
        struct_ident,
        struct_field_vec.as_slice(),
        struct_query_field_vec.as_slice(),
        struct_tree_prefix.is_some(),
    );

    /// 构建结构体-orm 增删改查方法
    let model_valid_token = build_struct_valid_token(struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-handler-web接口方法
    let model_handler_token =
        build_struct_handler_token(struct_ident, struct_tree_prefix.is_some());
    /// 构建结构体-router-路由方法
    let model_router_token = build_struct_router_token(struct_ident, struct_tree_prefix.is_some());
    model_struct_macro_token.push(model_struct_query_token);
    model_struct_macro_token.push(mode_struct_query_impl_token);
    model_struct_macro_token.push(model_struct_token);
    model_struct_macro_token.push(model_impl_get_set_token);
    model_struct_macro_token.push(model_impl_orm_table_token);
    model_struct_macro_token.push(model_impl_sql_token);
    model_struct_macro_token.push(model_from_rdbc_orm_row_token);
    model_struct_macro_token.push(model_orm_token);
    model_struct_macro_token.push(model_curd_token);
    model_struct_macro_token.push(model_valid_token);
    model_struct_macro_token.push(model_handler_token);
    model_struct_macro_token.push(model_router_token);
    let final_token = quote! {
       #(#model_struct_macro_token)*
    };
    println!("最终输出{}", final_token.to_string());
    final_token.into()
}

fn get_field_by_attrs(field_slice: &[Field], attrs: &str) -> Vec<Field> {
    let mut field_vec = vec![];
    for field in field_slice {
        if has_rdbc_attr(field, attrs) {
            field_vec.push(field.clone());
        }
    }
    field_vec
}

/// 构建结构体-数据校验方法
fn build_struct_valid_token(struct_ident: &Ident, struct_field_slice: &[Field]) -> TokenStream2 {
    let (insert_valid, update_valid) = get_valid_field(struct_field_slice);
    println!("===insert=======>{}", insert_valid.len());
    println!("===update=======>{}", update_valid.len());
    quote! {
        impl #struct_ident {
            pub fn insert_valid(&self) -> BmbpResp<()> {
                Ok(())
            }
            pub fn update_valid(&self) -> BmbpResp<()> {
                Ok(())
            }
        }
    }
}

fn build_struct_from_rdbc_orm_row_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
) -> TokenStream2 {
    let mut field_set_value_vec = vec![];
    for field in struct_fields {
        if has_rdbc_attr(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("set_{}", field_ident);
        let t_ = quote! {
          if let Some(data) = row.get_data().get(#field_name) {
              model.#field_method(Some(data.into()));
          }
        };
        field_set_value_vec.push(t_);
    }

    let final_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::new();
                #(#field_set_value_vec)*
                model
            }
        }
    };
    final_token
}

/// build_struct_field_vec 构建字段集合
fn build_struct_field_vec(
    base_struct_input: &DeriveInput,
    model_struct_input: &DeriveInput,
    tree_input: &DeriveInput,
    struct_tree_prefix: &Option<String>,
) -> Vec<Field> {
    let struct_ident = &model_struct_input.ident;
    // 结构体字段
    let mut model_field = get_struct_field(model_struct_input);
    let model_field_map = get_struct_field_name_map(&model_field);
    // 公共字段
    let base_field = get_struct_field(base_struct_input);
    for mut field in base_field {
        let field_name = field.ident.as_mut().unwrap().to_string();
        if !model_field_map.contains_key(field_name.as_str()) {
            model_field.push(field);
        }
    }
    // 公共树字段
    if let Some(tree_prefix) = struct_tree_prefix {
        let tree_field = get_struct_field(tree_input);
        for mut field in tree_field {
            let mut field_name = field.ident.as_mut().unwrap().to_string();
            if field_name.eq("children") {
                field.ty = parse_quote! { Option<Vec<#struct_ident>> };
            } else {
                let field_ident = format_ident!("{}_{}", tree_prefix, field_name);
                field.ident = Some(field_ident);
                let field_name = field.ident.as_mut().unwrap().to_string();
                if !model_field_map.contains_key(field_name.as_str()) {
                    model_field.push(field);
                }
            }
        }
    }
    model_field
}

fn build_struct_model_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_model_fields = build_struct_field_token(struct_fields);
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
    let struct_init_data_token = build_struct_impl_init_data_method_token_vec(struct_fields);
    let token = quote! {
        impl #struct_ident {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
            #(#struct_init_data_token)*
        }
    };
    token
}

fn build_struct_query_model_token(
    struct_query_ident: &Ident,
    query_fields: &[Field],
) -> TokenStream2 {
    let struct_query_field = build_struct_field_token(query_fields);
    quote! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_query_ident {
               #(#struct_query_field,)*
        }
    }
}

fn build_struct_query_impl_token(
    struct_query_ident: &Ident,
    struct_fields: &[Field],
) -> TokenStream2 {
    let struct_method_token = build_struct_impl_method_token_vec(struct_fields);
    let token = quote! {
        impl #struct_query_ident {
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
fn build_struct_impl_init_data_method_token_vec(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    let mut insert_data_value = vec![];
    let mut update_data_value = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let field_name_string = field_name.to_string();
        match field_name_string.as_str() {
            RDBC_DATA_ID => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(uuid::Uuid::new_v4().to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_LEVEL => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_FLAG => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }

            RDBC_DATA_STATUS => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_SORT => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(0);
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_CREATE_TIME => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_CREATE_USER => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_UPDATE_TIME => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                };
                insert_data_value.push(token.clone());
                update_data_value.push(token);
            }
            RDBC_DATA_UPDATE_USER => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
                update_data_value.push(token);
            }
            RDBC_DATA_OWNER_ORG => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
            }
            RDBC_DATA_SIGN => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
            }
            &_ => {}
        }
    }
    method_vec.push(quote! {
        pub fn init_insert_data(&mut self)->&mut Self{
            #(#insert_data_value)*
            self
        }
    });
    method_vec.push(quote! {
        pub fn init_update_data(&mut self)->&mut Self{
            #(#update_data_value)*
            self
        }
    });
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

fn build_struct_impl_script_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let insert_sql_token = build_struct_sql_script_insert_token(struct_fields);
    let update_sql_token = build_struct_sql_script_update_token(struct_fields);
    let sql_token = quote! {
        impl #struct_ident {
            pub fn build_query_sql() -> Query {
                let mut query = Query::new();
                query.table(Self::get_table_name());
                query.select_vec(Self::get_table_columns());
                query.order_by("data_sort", true);
                query.order_by("data_update_time", false);
                query
            }
            pub fn build_info_sql(data_id:&Option<String>) -> Query {
                let mut query = Query::new();
                query.table(Self::get_table_name());
                query.select_vec(Self::get_table_columns());
                query.eq_(Self::get_table_primary_key(), data_id);
                query
            }
            pub fn build_remove_sql(data_id:&Option<String>) -> Delete {
                let mut delete = Delete::new();
                delete.table(Self::get_table_name());
                delete.eq_(Self::get_table_primary_key(), data_id);
                delete
            }
            pub fn build_enable_sql(data_id:&Option<String>) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update.set("data_status", "1");
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_disable_sql(data_id:&Option<String>) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update.set("data_status", "0");
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_update_status_sql(data_id:&Option<String>, status: String ) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update.set("data_status", status);
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_update_flag_sql(data_id:&Option<String>, flag: String) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                update.set("data_flag", flag);
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            #insert_sql_token
            #update_sql_token
        }
    };
    sql_token
}

fn build_struct_sql_script_update_token(struct_fields: &[Field]) -> TokenStream2 {
    let mut update_field_vec = vec![];
    for field in struct_fields {
        if has_rdbc_attr(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("get_{}", field_ident);
        let insert_item = quote! {
            if let Some(value) = self.#field_method() {
                update.set(#field_name, value);
            }
        };
        update_field_vec.push(insert_item);
    }
    quote! {
        pub fn build_update_sql(&self) -> Update {
                let mut update = Update::new();
                update.table(Self::get_table_name());
                #(#update_field_vec)*
                update.eq_(Self::get_table_primary_key(),self.get_data_id());
                update
            }
    }
}

fn build_struct_sql_script_insert_token(struct_fields: &[Field]) -> TokenStream2 {
    let mut insert_field_vec = vec![];
    for field in struct_fields {
        if has_rdbc_attr(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("get_{}", field_ident);
        let insert_item = quote! {
            if let Some(value) = self.#field_method() {
                insert.insert_column_value(#field_name, value);
            }
        };
        insert_field_vec.push(insert_item);
    }
    quote! {
        pub fn build_insert_sql(&self) -> Insert {
                let mut insert = Insert::new();
                insert.table(Self::get_table_name());
                #(#insert_field_vec)*
                insert
            }
    }
}

fn build_struct_orm_token(struct_ident: &Ident) -> TokenStream2 {
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let token = quote! {
        pub struct #orm_ident;
        impl #orm_ident {
            pub async fn select_page_by_query(
                page_no: &usize,
                page_size: &usize,
                query: &Query,
            ) -> BmbpResp<PageVo<#struct_ident>> {
                match RdbcORM
                    .await
                    .select_page_by_query::<#struct_ident>(page_no.clone(), page_size.clone(), query)
                    .await
                {
                    Ok(mut page) => {
                        let mut page_vo = PageVo::new();
                        page_vo.set_page_no(page.page_num().clone());
                        page_vo.set_page_size(page.page_size().clone());
                        page_vo.set_op_data(page.data_take());
                        page_vo.set_row_total(page.total().clone());
                        Ok(page_vo)
                    }
                    Err(e) => Err(BmbpError::service(e.get_msg().as_str())),
                }
            }
                pub async fn select_list_by_query(query: &Query) -> BmbpResp<Option<Vec<#struct_ident>>> {
                    match RdbcORM
                        .await
                        .select_list_by_query::<#struct_ident>(query)
                        .await
                    {
                        Ok(data) => Ok(data),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn select_one_by_query(query: &Query) -> BmbpResp<Option<#struct_ident>> {
                    match RdbcORM
                        .await
                        .select_one_by_query::<#struct_ident>(query)
                        .await
                    {
                        Ok(data) => Ok(data),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_insert(insert: &Insert) -> BmbpResp<usize> {
                    match RdbcORM.await.execute_insert(insert).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_update(update: &Update) -> BmbpResp<usize> {
                    match RdbcORM.await.execute_update(update).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_delete(delete: &Delete) -> BmbpResp<usize> {
                    match RdbcORM.await.execute_delete(delete).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
        }
    };
    token
}

fn build_struct_curd_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
    struct_query_fields: &[Field],
    is_tree: bool,
) -> TokenStream2 {
    let struct_query_filter_sql_token = build_struct_query_filter_token(struct_query_fields);
    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);
    let orm_ident = format_ident!("{}Orm", struct_ident);

    let mut tree_method_vec = vec![];
    if is_tree {
        tree_method_vec = vec![
            quote! {
                pub async fn find_tree(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_by_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_by_code(code: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_by_parent_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_by_parent_code(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                 pub async fn find_tree_by_code_path(code_path: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_exclude(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_by_exclude_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_exclude_by_code(code: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_exclude_by_parent_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                pub async fn find_tree_exclude_by_parent_code(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
            quote! {
                 pub async fn find_tree_exclude_by_code_path(code_path: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
                   Ok(vec![])
                }
            },
        ]
    }

    let token = quote! {
        impl #struct_ident {
            #(#tree_method_vec)*
            pub async fn find_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                query.eq_("data_flag","0");
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_all_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_removed_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                query.eq_("data_flag","-1");
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_page_by_query(page_no: &usize, page_size: &usize,query:&Query) -> BmbpResp<PageVo<Self>> {
                #orm_ident::select_page_by_query(page_no, page_size, &query).await
            }

            pub async fn find_list(query_vo: #struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
                query.eq_("data_flag","0");
                Self::find_list_by_query(&query).await
            }
            pub async fn find_all_list(query_vo:#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
               Self::find_list_by_query(&query).await
            }
            pub async fn find_removed_list(query_vo:#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
                query.eq_("data_flag","-1");
                Self::find_list_by_query(&query).await
            }
            pub async fn find_list_by_query(query:&Query)-> BmbpResp<Option<Vec<Self>>> {
                #orm_ident::select_list_by_query(query).await
            }
            pub async fn find_by_id(id: &Option<String>) -> BmbpResp<Option<Self>> {
                let mut query = #struct_ident::build_query_sql();
                query.eq_(Self::get_table_primary_key(),id);
                #orm_ident::select_one_by_query(&query).await
            }

            pub async fn find_one(&self) -> BmbpResp<Option<Self>> {
                Self::find_by_id(&self.get_data_id()).await
            }
            pub async fn save(&mut self) -> BmbpResp<Option<Self>> {
                let model = self.find_one().await?;
                if model.is_some() {
                    self.update().await?;
                } else {
                    self.insert().await?;
                }
                self.find_one().await
            }
            pub async fn insert(&mut self) -> BmbpResp<usize> {
                // 初始化数据
                self.init_insert_data();
                let insert = self.build_insert_sql();
                #orm_ident::execute_insert(&insert).await
            }
            pub async fn update(&mut self) -> BmbpResp<usize> {
                self.init_update_data();
                let update = self.build_update_sql();
                #orm_ident::execute_update(&update).await
            }
            pub async fn remove(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let remove = Self::build_remove_sql(self.get_data_id());
                #orm_ident::execute_delete(&remove).await
            }
            pub async fn remove_logic(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let update = Self::build_update_flag_sql(self.get_data_id(),"-1".to_string());
                #orm_ident::execute_update(&update).await
            }
            pub async fn enable(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要启用的记录"));
                }
                let update = Self::build_enable_sql(self.get_data_id());
                #orm_ident::execute_update(&update).await
            }
            pub async fn disable(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要停用的记录"));
                }
                let update = Self::build_disable_sql(self.get_data_id());
                #orm_ident::execute_update(&update).await
            }


            pub async fn remove_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let update = Self::build_disable_sql(id);
                #orm_ident::execute_update(&update).await
            }
            pub async fn remove_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
             pub async fn remove_logic_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let update = Self::build_update_flag_sql(id,"-1".to_string());
                #orm_ident::execute_update(&update).await
            }
            pub async fn remove_logic_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn enable_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要启用的记录"));
                }
                let update = Self::build_enable_sql(id);
                #orm_ident::execute_update(&update).await
            }
            pub async fn enable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn disable_by_id(id: &Option<String>) -> BmbpResp<usize> {
                 if id.is_none() {
                    return Err(BmbpError::service("请指定要停用的记录"));
                }
                let update = Self::build_disable_sql(id);
                #orm_ident::execute_update(&update).await
            }
            pub async fn disable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
        }
    };
    token
}

fn build_struct_query_filter_token(query_fields: &[Field]) -> Vec<TokenStream2> {
    let mut token_vec = vec![];
    for field in query_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let get_method_name = format_ident!("get_{}", field_ident);
        let query_type: String = get_query_type(field);
        match query_type.as_str() {
            "eq" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.eq_(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "ne" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.ne_(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like_left" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like_left_value(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like_right" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like_right_value(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            _ => {}
        }
    }
    token_vec
}

fn build_struct_handler_token(struct_ident: &Ident, is_tree: bool) -> TokenStream2 {
    let struct_name = camel_to_snake(struct_ident.to_string()).to_lowercase();
    let find_all_page_name = format_ident!("{}_find_all_page", struct_name);
    let find_removed_page_name = format_ident!("{}_find_removed_page", struct_name);
    let find_page_name = format_ident!("{}_find_page", struct_name);
    let find_all_list_name = format_ident!("{}_find_all_list", struct_name);
    let find_removed_list_name = format_ident!("{}_find_removed_list", struct_name);
    let find_list_name = format_ident!("{}_find_list", struct_name);
    let find_info_name = format_ident!("{}_find_info", struct_name);
    let save_name = format_ident!("{}_save", struct_name);
    let save_batch_name = format_ident!("{}_save_batch", struct_name);
    let insert_name = format_ident!("{}_insert", struct_name);
    let insert_batch_name = format_ident!("{}_insert_batch", struct_name);
    let update_name = format_ident!("{}_update", struct_name);
    let update_batch_name = format_ident!("{}_update_batch", struct_name);
    let remove_name = format_ident!("{}_remove", struct_name);
    let remove_batch_name = format_ident!("{}_remove_batch", struct_name);
    let remove_logic_name = format_ident!("{}_remove_logic", struct_name);
    let remove_logic_batch_name = format_ident!("{}_remove_logic_batch", struct_name);
    let enable_name = format_ident!("{}_enable", struct_name);
    let disable_name = format_ident!("{}_disable", struct_name);

    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);

    let mut tree_handler = vec![];
    if (is_tree) {
        let find_tree_name = format_ident!("{}_find_tree", struct_name);
        let find_tree_by_id_name = format_ident!("{}_find_tree_by_id", struct_name);
        let find_tree_by_code = format_ident!("{}_find_tree_by_code", struct_name);
        let find_tree_by_parent_code = format_ident!("{}_find_tree_by_parent_code", struct_name);
        let find_tree_by_parent_id = format_ident!("{}_find_tree_by_parent_id", struct_name);
        let find_tree_by_code_path = format_ident!("{}_find_tree_by_code_path", struct_name);

        let find_tree_exclude_name = format_ident!("{}_find_tree_exclude", struct_name);
        let find_tree_exclude_by_id_name = format_ident!("{}_find_tree_exclude_by_id", struct_name);
        let find_tree_exclude_by_code = format_ident!("{}_find_tree_exclude_by_code", struct_name);
        let find_tree_exclude_by_parent_code =
            format_ident!("{}_find_tree_exclude_by_parent_code", struct_name);
        let find_tree_exclude_by_parent_id =
            format_ident!("{}_find_exclude_tree_by_parent_id", struct_name);
        let find_tree_exclude_by_code_path =
            format_ident!("{}_find_exclude_tree_by_code_path", struct_name);
        tree_handler = vec![
            quote! {
                #[handler]
                pub async fn #find_tree_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_id_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_parent_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_parent_id(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_code_path(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_id_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_parent_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_parent_id(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_code_path(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
        ]
    }
    quote! {
        #(#tree_handler)*

        #[handler]
        pub async fn #find_all_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_all_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_removed_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_removed_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_removed_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_all_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_all_list(query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_removed_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_removed_list(query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_list(query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_info_name(req: &mut Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            let data_id = req.param::<String>("dataId");
            let model_vo = #struct_ident::find_by_id(&data_id).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }
        #[handler]
        pub async fn #save_name(req: &mut Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            let mut save_vo = req.parse_json::<#struct_ident>().await?;
            Ok(RespVo::ok_save_option(save_vo.save().await?))
        }
        #[handler]
        pub async fn #save_batch_name(req: &mut Request, resp: &mut Response, ) ->HttpRespListVo<#struct_ident>  {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #insert_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #insert_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #update_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #update_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #remove_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::remove_by_id(&data_id).await?;
             Ok(RespVo::ok_remove_data(row_count))
        }
        #[handler]
        pub async fn #remove_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             Ok(RespVo::ok_remove_option(None))
        }
        #[handler]
        pub async fn #remove_logic_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::remove_by_id(&data_id).await?;
             Ok(RespVo::ok_remove_data(row_count))
        }
        #[handler]
        pub async fn #remove_logic_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
              Ok(RespVo::ok_remove_option(None))
        }
        #[handler]
        pub async fn #enable_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::enable_by_id(&data_id).await?;
              Ok(RespVo::ok_enable_data(row_count))
        }
        #[handler]
        pub async fn #disable_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
              let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::disable_by_id(&data_id).await?;
              Ok(RespVo::ok_disable_data(row_count))
        }
    }
}

fn build_struct_router_token(struct_ident: &Ident, is_tree: bool) -> TokenStream2 {
    let struct_name = camel_to_snake(struct_ident.to_string()).to_lowercase();
    let find_all_page_name = format_ident!("{}_find_all_page", struct_name);
    let find_removed_page_name = format_ident!("{}_find_removed_page", struct_name);
    let find_page_name = format_ident!("{}_find_page", struct_name);
    let find_all_list_name = format_ident!("{}_find_all_list", struct_name);
    let find_removed_list_name = format_ident!("{}_find_removed_list", struct_name);
    let find_list_name = format_ident!("{}_find_list", struct_name);
    let find_info_name = format_ident!("{}_find_info", struct_name);
    let save_name = format_ident!("{}_save", struct_name);
    let save_batch_name = format_ident!("{}_save_batch", struct_name);
    let insert_name = format_ident!("{}_insert", struct_name);
    let insert_batch_name = format_ident!("{}_insert_batch", struct_name);
    let update_name = format_ident!("{}_update", struct_name);
    let update_batch_name = format_ident!("{}_update_batch", struct_name);
    let remove_name = format_ident!("{}_remove", struct_name);
    let remove_batch_name = format_ident!("{}_remove_batch", struct_name);
    let remove_logic_name = format_ident!("{}_remove_logic", struct_name);
    let remove_logic_batch_name = format_ident!("{}_remove_logic_batch", struct_name);
    let enable_name = format_ident!("{}_enable", struct_name);
    let disable_name = format_ident!("{}_disable", struct_name);
    let mut tree_router = vec![];
    if (is_tree) {
        let find_tree_name = format_ident!("{}_find_tree", struct_name);
        let find_tree_by_id_name = format_ident!("{}_find_tree_by_id", struct_name);
        let find_tree_by_code = format_ident!("{}_find_tree_by_code", struct_name);
        let find_tree_by_parent_code = format_ident!("{}_find_tree_by_parent_code", struct_name);
        let find_tree_by_parent_id = format_ident!("{}_find_tree_by_parent_id", struct_name);
        let find_tree_by_code_path = format_ident!("{}_find_tree_by_code_path", struct_name);

        let find_tree_exclude_name = format_ident!("{}_find_tree_exclude", struct_name);
        let find_tree_exclude_by_id_name = format_ident!("{}_find_tree_exclude_by_id", struct_name);
        let find_tree_exclude_by_code = format_ident!("{}_find_tree_exclude_by_code", struct_name);
        let find_tree_exclude_by_parent_code =
            format_ident!("{}_find_tree_exclude_by_parent_code", struct_name);
        let find_tree_exclude_by_parent_id =
            format_ident!("{}_find_exclude_tree_by_parent_id", struct_name);
        let find_tree_exclude_by_code_path =
            format_ident!("{}_find_exclude_tree_by_code_path", struct_name);

        tree_router.push(quote! {
             .push(Router::with_path("/find/tree").post(#find_tree_name))
             .push(Router::with_path("/find/tree/id/<dataId>").post(#find_tree_by_id_name))
             .push(Router::with_path("/find/tree/code/<code>").post(#find_tree_by_code))
             .push(Router::with_path("/find/tree/parent/id/<dataId>").post(#find_tree_by_parent_id))
             .push(Router::with_path("/find/tree/parent/code/<code>").post(#find_tree_by_parent_code))
             .push(Router::with_path("/find/tree/path/<codePath>").post(#find_tree_by_code_path))
             .push(Router::with_path("/find/tree/exclude").post(#find_tree_exclude_name))
             .push(Router::with_path("/find/tree/exclude/id/<dataId>").post(#find_tree_exclude_by_id_name))
             .push(Router::with_path("/find/tree/exclude/code/<code>").post(#find_tree_exclude_by_code))
             .push(Router::with_path("/find/tree/exclude/parent/id/<dataId>").post(#find_tree_exclude_by_parent_id))
             .push(Router::with_path("/find/tree/exclude/parent/code/<code>").post(#find_tree_exclude_by_parent_code))
             .push(Router::with_path("/find/tree/exclude/path/<codePath>").post(#find_tree_exclude_by_code_path))
        });
    }

    quote! {
        impl #struct_ident {
            pub fn build_router() -> Router {
               Router::new()
                .push(Router::with_path("/find/all/page").post(#find_all_page_name))
                .push(Router::with_path("/find/removed/page").post(#find_removed_page_name))
                .push(Router::with_path("/find/page").post(#find_page_name))
                .push(Router::with_path("/find/all/list").post(#find_all_list_name))
                .push(Router::with_path("/find/removed/list").post(#find_removed_list_name))
                .push(Router::with_path("/find/list").post(#find_list_name))
                .push(Router::with_path("/find/info/id/<dataId>").post(#find_info_name))
                .push(Router::with_path("/save").post(#save_name))
                .push(Router::with_path("/save/batch").post(#save_batch_name))
                .push(Router::with_path("/insert").post(#insert_name))
                .push(Router::with_path("/insert/batch").post(#insert_batch_name))
                .push(Router::with_path("/update").post(#update_name))
                .push(Router::with_path("/update/batch").post(#update_batch_name))
                .push(Router::with_path("/remove/id/<dataId>").post(#remove_name))
                .push(Router::with_path("/remove/batch/id").post(#remove_batch_name))
                 .push(Router::with_path("/remove/logic/id/<dataId>").post(#remove_logic_name))
                .push(Router::with_path("/remove/logic/batch/id").post(#remove_logic_batch_name))
                .push(Router::with_path("/enable/id/<dataId>").post(#enable_name))
                .push(Router::with_path("/disable/id/<dataId>").post(#disable_name))
                #(#tree_router)*
            }
        }
    }
}
