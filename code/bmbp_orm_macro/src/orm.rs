use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Field, Ident, ItemStruct, Meta};

/// OrmMeta #[orm(table=xxx,id=XX)]
#[derive(Debug)]
struct OrmMeta {
    table_name: String,
    id_name: String,
}
pub fn orm(orm_meta_token: TokenStream, orm_struct_token: TokenStream) -> TokenStream {
    // 解析#[orm(table=xxx,id=xxx)] 获取表、主键字段
    let mut orm_meta = get_orm_meta(&orm_meta_token);

    // 处理ORM的公共属性
    let mut orm_model_base_field = build_orm_model_base_field();
    let mut orm_model_base_field_token = vec![];
    let mut orm_model_base_method_token = vec![];
    for field_name in orm_model_base_field.as_slice() {
        let field_ident = format_ident!("{}", field_name.clone());
        orm_model_base_field_token.push(quote!(
           #field_ident :String
        ));
        let set_method_ident = format_ident!("set_{}", field_name.clone());
        let get_method_ident = format_ident!("get_{}", field_name.clone());
        let get_mut_method_ident = format_ident!("get_mut_{}", field_name.clone());
        orm_model_base_method_token.push(quote!(
                    pub fn #set_method_ident(&mut self,v : String)->&mut Self{
                        self.#field_ident = v;
                        self
                    }
                    pub fn #get_method_ident(&mut self)->&String{
                        &self.#field_ident
                    }
                    pub fn #get_mut_method_ident(&mut self)->&mut String{
                        &mut self.#field_ident
                    }
        ));
    }

    // 处理Struct定义的属性
    let orm_struct_ident = parse_macro_input!(orm_struct_token as DeriveInput);
    let orm_struct_name = &orm_struct_ident.ident.to_string().replace("\"", "");
    // 处理表名
    if orm_meta.table_name.is_empty() {
        orm_meta.table_name = camel_to_snake(orm_struct_name.clone());
    }

    let mut orm_struct_field = vec![];
    let mut orm_struct_field_token = vec![];
    let mut orm_struct_method_token = vec![];

    let orm_struct_ident_data = &orm_struct_ident.data;
    match orm_struct_ident_data {
        Data::Struct(orm_struct_data) => {
            let orm_struct_fields = &orm_struct_data.fields;
            for struct_field in orm_struct_fields {
                let struct_field_name = &struct_field.ident.as_ref().unwrap().to_string();
                if !is_orm_struct_field_has_skip_meta(struct_field) {
                    orm_struct_field.push(struct_field_name.clone());
                }
                let orm_struct_field_ident = struct_field.ident.as_ref().unwrap();
                let orm_struct_field_type = &struct_field.ty;

                // 增加结构字段
                orm_struct_field_token.push(quote!(
                    #orm_struct_field_ident : #orm_struct_field_type
                ));

                let set_field_method_ident = format_ident!("set_{}", orm_struct_field_ident);
                let get_field_method_ident = format_ident!("get_{}", orm_struct_field_ident);
                let get_mut_field_ident = format_ident!("get_mut_{}", orm_struct_field_ident);
                let field_method_token = quote!(
                    pub fn #set_field_method_ident(&mut self,v : #orm_struct_field_type)->&mut Self{
                        self.#orm_struct_field_ident = v;
                        self
                    }
                    pub fn #get_field_method_ident(&mut self)->& #orm_struct_field_type{
                        &self.#orm_struct_field_ident
                    }
                    pub fn #get_mut_field_ident(&mut self)->& mut #orm_struct_field_type{
                        &mut self.#orm_struct_field_ident
                    }
                );
                orm_struct_method_token.push(field_method_token.to_token_stream());
            }
        }
        _ => {}
    }

    // 拼接StructField
    let mut all_orm_struct_field = vec![];
    all_orm_struct_field.extend_from_slice(orm_model_base_field.as_slice());
    all_orm_struct_field.extend_from_slice(orm_struct_field.as_slice());

    // 合并属性
    let mut all_orm_struct_field_token = vec![];
    all_orm_struct_field_token.extend_from_slice(orm_model_base_field_token.as_slice());
    all_orm_struct_field_token.extend_from_slice(orm_struct_field_token.as_slice());

    // 合并method
    let mut all_orm_struct_method_token = vec![];
    all_orm_struct_method_token.extend_from_slice(orm_model_base_method_token.as_slice());
    all_orm_struct_method_token.extend_from_slice(orm_struct_method_token.as_slice());

    // 增加 获取表结构、获取表字段方法
    let orm_table_name = orm_meta.table_name.clone();
    let orm_get_table_method_token = quote!(
        pub fn get_orm_table() -> String {
             #orm_table_name.to_string()
        }
    );
    all_orm_struct_method_token.push(orm_get_table_method_token);

    // 增加获取驼峰字段属性方法
    let mut all_orm_struct_camel_field = vec![];
    for item in all_orm_struct_field.as_slice() {
        all_orm_struct_camel_field.push(snake_to_camel(item.clone()));
    }

    // 增加获取字段属性方法
    let orm_get_field_method_token = quote!(
        pub fn get_orm_fields() -> Vec<String> {
            vec![
                #(#all_orm_struct_field.to_string()),*
            ]
        }
        pub fn get_orm_camel_fields() -> Vec<String> {
            vec![
                #(#all_orm_struct_camel_field.to_string()),*
            ]
        }
    );
    all_orm_struct_method_token.push(orm_get_field_method_token);

    // 拼pu接Struct类
    let struct_name_ident = &orm_struct_ident.ident;

    let mut orm_query_sql_token = quote!(
        pub fn query_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.from(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            for item in fields.as_slice() {
                script_sql.select(item);
            }
            script_sql
        }
        pub fn query_camel_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.from(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            let camel_fields: Vec<String> = #struct_name_ident::get_orm_camel_fields();
            for (index, item) in fields.as_slice().into_iter().enumerate() {
               let camel_field = camel_fields.get(index);
                match camel_field {
                    Some(v)=>{
                        script_sql.select(format!("{} as \"{}\"",item,v).as_str());

                    }
                    None=>{
                     script_sql.select(format!("{} as \"{}\"",item,item).as_str());
                    }
                }
            }
            script_sql
        }
    );
    all_orm_struct_method_token.push(orm_query_sql_token);

    let mut orm_update_sql_token = quote!(
        pub fn update_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.update(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            for item in fields.as_slice() {
                script_sql.set(format!("{}=#{{{}}}",item,item).as_str());
            }
            script_sql
        }
        pub fn update_camel_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.update(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            let camel_fields: Vec<String> = #struct_name_ident::get_orm_camel_fields();
            for (index, item) in fields.as_slice().into_iter().enumerate() {
               let camel_field = camel_fields.get(index);
                match camel_field {
                    Some(v)=>{
                        script_sql.set(format!("{}=#{{{}}}",item,v).as_str());
                    }
                    None=>{
                        script_sql.set(format!("{}=#{{{}}}",item,item).as_str());
                    }
                }
            }
            script_sql
        }
    );
    all_orm_struct_method_token.push(orm_update_sql_token);
    let mut orm_meta_id_name = orm_meta.id_name.clone();
    if orm_meta_id_name.is_empty() {
        orm_meta_id_name = "r_id".to_string();
    }

    let filter_ident = format!(
        "{}=#{{{}}}",
        camel_to_snake(orm_meta_id_name.clone()).to_lowercase(),
        snake_to_camel(camel_to_snake(orm_meta_id_name.clone()))
    );
    let mut orm_delete_sql_token = quote!(
        pub fn delete_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.delete(#struct_name_ident::get_orm_table().as_str());
            script_sql
        }
        pub fn delete_one_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.delete(#struct_name_ident::get_orm_table().as_str());
            script_sql.filter("r_id=#{rId}");
            script_sql
        }
        pub fn delete_by_id_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.delete(#struct_name_ident::get_orm_table().as_str());
            script_sql.filter(#filter_ident);
            script_sql
        }
    );
    all_orm_struct_method_token.push(orm_delete_sql_token);

    let mut orm_insert_sql_token = quote!(
        pub fn insert_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.insert_into(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            for item in fields.as_slice() {
                script_sql.insert_value(item, format!("#{{{}}}", item).as_str());
            }
            script_sql
        }
        pub fn insert_camel_sql() -> BmbpScriptSql {
            let mut script_sql = BmbpScriptSql::new();
            script_sql.insert_into(#struct_name_ident::get_orm_table().as_str());
            let fields: Vec<String> = #struct_name_ident::get_orm_fields();
            let camel_fields: Vec<String> = #struct_name_ident::get_orm_camel_fields();
            for (index, item) in fields.as_slice().into_iter().enumerate() {
               let camel_field = camel_fields.get(index);
                match camel_field {
                    Some(v)=>{
                        script_sql.insert_value(item, format!("#{{{}}}", v).as_str());
                    }
                    None=>{
                        script_sql.insert_value(item, format!("#{{{}}}", item).as_str());
                    }
                }
            }
            script_sql
        }
    );
    all_orm_struct_method_token.push(orm_insert_sql_token);

    let new_struct_token = quote!(
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_name_ident{
             #(#all_orm_struct_field_token,)*
        }
        impl #struct_name_ident{
            #(#all_orm_struct_method_token)*
        }
    );
    new_struct_token.into()
}

fn get_orm_meta(token_stream: &TokenStream) -> OrmMeta {
    let mut orm_token = OrmMeta {
        table_name: "".to_string(),
        id_name: "".to_string(),
    };
    let mut token_tree = vec![];
    for item in token_stream.clone().into_iter() {
        token_tree.push(item);
    }
    match token_tree.len() {
        0 => {}
        1 => set_orm_meta_table(&mut orm_token, &token_tree[0]),
        3 => set_orm_meta(
            &mut orm_token,
            &token_tree[0],
            &token_tree[1],
            &token_tree[2],
        ),
        7 => {
            let split = get_token_tree_node_name(&token_tree[3]);
            if "," == split.as_str() {
                set_orm_meta(
                    &mut orm_token,
                    &token_tree[0],
                    &token_tree[1],
                    &token_tree[2],
                );
                set_orm_meta(
                    &mut orm_token,
                    &token_tree[4],
                    &token_tree[5],
                    &token_tree[6],
                );
            }
        }
        _ => {}
    }
    orm_token
}

fn set_orm_meta_table(orm: &mut OrmMeta, token: &TokenTree) {
    match token {
        TokenTree::Ident(v) => {
            orm.table_name = v.to_string().replace("\"", "");
        }
        TokenTree::Literal(v) => {
            orm.table_name = v.to_string().replace("\"", "");
        }
        _ => {}
    }
}

fn set_orm_meta(orm: &mut OrmMeta, left: &TokenTree, split: &TokenTree, right: &TokenTree) {
    let split_str = split.to_string();
    match split_str.as_str() {
        "=" | "," | "" => {
            let left_v = get_token_tree_node_name(left);
            let right_v = get_token_tree_node_name(right);
            match left_v.as_str() {
                "table" => {
                    orm.table_name = right_v;
                }
                "id" => {
                    orm.id_name = right_v;
                }
                _ => {
                    orm.table_name = left_v;
                    orm.id_name = right_v;
                }
            }
        }
        _ => {}
    }
}

fn get_token_tree_node_name(token: &TokenTree) -> String {
    match token {
        TokenTree::Ident(v) => v.to_string().replace("\"", ""),
        TokenTree::Literal(v) => v.to_string().replace("\"", ""),
        _ => "".to_string(),
    }
}

fn is_orm_struct_field_has_skip_meta(field: &Field) -> bool {
    // 从属性里面提取宏
    let field_attrs = field.attrs.as_slice();
    if field_attrs.is_empty() {
        return false;
    };
    let mut has_skip_marco = false;
    for attr in field_attrs {
        let meta = &attr.meta;
        match meta {
            Meta::Path(mv) => {
                let temp = mv.to_token_stream().to_string();
                if "skip".eq(temp.as_str()) {
                    has_skip_marco = true;
                    break;
                }
            }
            Meta::List(v) => {
                println!("meta2:{:#?}", v.to_token_stream());
            }
            Meta::NameValue(v) => {
                println!("meta3:{:#?}", v.to_token_stream());
            }
        }
    }
    has_skip_marco
}

/// orm_model_common_field 增加公共字段
fn build_orm_model_base_field() -> Vec<String> {
    vec![
        "r_id".to_string(),
        "r_level".to_string(),
        "r_flag".to_string(),
        "r_create_time".to_string(),
        "r_create_user".to_string(),
        "r_update_time".to_string(),
        "r_update_user".to_string(),
        "r_owner_org".to_string(),
        "r_owner_user".to_string(),
        "r_sign".to_string(),
    ]
}

// 字符串转换
fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}

fn snake_to_camel(snake_string: String) -> String {
    case_style::CaseStyle::from_snakecase(snake_string)
        .to_camelcase()
        .to_string()
}
