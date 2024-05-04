use proc_macro::{TokenStream, TokenTree};

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field};

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
    let model_impl_sql_token =
        build_struct_impl_script_token(&struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-RdbcOrmRow的转换
    let model_from_rdbc_orm_row_token =
        build_struct_from_rdbc_orm_row_token(struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-orm 操作数据库
    let model_orm_token = build_struct_orm_token(&struct_ident);

    /// 构建结构体-orm 增删改查方法
    let model_curd_token = build_struct_curd_token(struct_ident, struct_field_vec.as_slice());

    /// 构建结构体-orm 增删改查方法
    let model_valid_token = build_struct_valid_token(struct_ident, struct_field_vec.as_slice());

    token_vec.push(model_struct_token);
    token_vec.push(model_impl_get_set_token);
    token_vec.push(model_impl_orm_table_token);
    token_vec.push(model_impl_sql_token);
    token_vec.push(model_from_rdbc_orm_row_token);
    token_vec.push(model_orm_token);
    token_vec.push(model_curd_token);
    token_vec.push(model_valid_token);
    let final_token = quote! {
       #(#token_vec)*
    };
    println!("最终输出{}", final_token.to_string());
    final_token.into()
}

/// 构建结构体-数据校验方法
fn build_struct_valid_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    quote! {
        impl #struct_ident {
            pub fn valid(&self) -> BmbpResp<()> {
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
                    .select_page_by_query::<#struct_ident>(page_no.clone(), page_size.clone(), &query)
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
                        .select_list_by_query::<#struct_ident>(&query)
                        .await
                    {
                        Ok(data) => Ok(data),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn select_one_by_query(query: &Query) -> BmbpResp<Option<#struct_ident>> {
                    match RdbcORM
                        .await
                        .select_one_by_query::<#struct_ident>(&query)
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

fn build_struct_curd_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let token = quote! {
        impl #struct_ident {
            pub async fn find_one(&self) -> BmbpResp<Option<Self>> {
                Ok(Some(self.clone()))
            }
            pub async fn save(&self) -> BmbpResp<Option<Self>> {
                let model = self.find_one().await?;
                if model.is_some() {
                    self.update().await?;
                } else {
                    self.insert().await?;
                }
                self.find_one().await
            }
            pub async fn insert(&self) -> BmbpResp<usize> {
                let insert = self.build_insert_sql();
                Ok(0)
            }
            pub async fn update(&self) -> BmbpResp<usize> {
                let insert = self.build_update_sql();
                Ok(0)
            }
            pub async fn remove(&self) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn remove_logic(&self) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn enable(&self) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn disable(&self) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn find_page(page_no: Option<&usize>, page_size: Option<&usize>) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_page_by_query(page_no: Option<&usize>, page_size: Option<&usize>,query:Query) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_list() -> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_list_by_query(query:&Query)-> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_removed_page(page_no: Option<&usize>, page_size: Option<&usize>) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_removed_page_by_query(page_no: Option<&usize>, page_size: Option<&usize>) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_removed_list() -> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_removed_list_by_query(query:&Query)-> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_all_page(page_no: Option<&usize>, page_size: Option<&usize>) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_all_page_by_query(page_no: Option<&usize>, page_size: Option<&usize>,query:Query) -> BmbpResp<PageVo<Self>> {
                Ok(PageVo::new())
            }
            pub async fn find_all_list() -> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_all_list_by_query(query:&Query)-> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_list_by_id_slice(id: Option<&[String]>) -> BmbpResp<Option<Vec<Self>>> {
                Ok(None)
            }
            pub async fn find_by_id(id: Option<&String>) -> BmbpResp<Option<Self>> {
                Ok(None)
            }
            pub async fn remove_by_id(id: Option<&String>) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn remove_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
             pub async fn remove_logic_by_id(id: Option<&String>) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn remove_logic_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn enable_by_id(id: Option<String>) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn enable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn disable_by_id(id: Option<String>) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn disable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }

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
