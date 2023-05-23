use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util;

/// OrmMeta #[orm(table=xxx,id=XX)]
#[derive(Debug)]
struct OrmMeta {
    table_name: String,
    id_name: String,
    exclude_field: Vec<String>,
}

impl OrmMeta {
    fn is_exclude_filed(&self, field: &String) -> bool {
        self.exclude_field.contains(field)
    }
}

pub fn orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    // 解析#[orm(table=xxx,id=xxx,exclude=children)] 获取表、主键字段
    let mut orm_meta = get_orm_meta(&meta_token);
    // 处理Struct定义的属性
    let derive_input = parse_macro_input!(struct_token as DeriveInput);
    let struct_name = &derive_input.ident;
    if orm_meta.table_name.is_empty() {
        orm_meta.table_name = camel_to_snake(struct_name.to_string()).to_uppercase();
    }
    let struct_field = util::parse_struct_field(&derive_input);
    let orm_field = struct_field
        .iter()
        .filter(|(x, _)| !orm_meta.is_exclude_filed(x))
        .collect::<Vec<_>>();

    let orm_field_name_vec: Vec<String> = orm_field.iter().map(|(x, _)| x.to_string()).collect();
    let orm_field_name_alias_vec: Vec<String> = orm_field
        .iter()
        .map(|(x, _)| (snake_to_camel(x.to_string())))
        .collect();
    let orm_table_name = orm_meta.table_name.clone();
    let orm_method = quote! {
        impl #struct_name{
            pub fn get_orm_table()->String{
                #orm_table_name.to_string()
            }

            pub fn get_orm_fields()->Vec<String>{
                vec![#(#orm_field_name_vec.to_string()),*]
            }

            pub fn get_orm_field_alias()->Vec<(String,String)>{
                vec![#((#orm_field_name_vec.to_string(),#orm_field_name_alias_vec.to_string())),*]
            }

        }
    };

    let orm_script_method = build_orm_script_method();
    let orm_script_struct = quote!(
        impl #struct_name{
            #(#orm_script_method)*
        }
    );

    quote!(
        #derive_input
        #orm_method
        #orm_script_struct
    )
    .into()
}

fn get_orm_meta(token_stream: &TokenStream) -> OrmMeta {
    let mut orm_meta = OrmMeta {
        table_name: "".to_string(),
        id_name: "".to_string(),
        exclude_field: vec![],
    };

    let meta_string = token_stream.to_string();
    if meta_string.trim().is_empty() {
        return orm_meta;
    }

    let trim_meta_string = meta_string.replace(" ", "");
    let tokens = trim_meta_string.split(",").collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if token.contains("=") {
            let t_v: Vec<String> = token
                .to_string()
                .split("=")
                .map(|x| x.to_string())
                .collect();
            if t_v.len() >= 2 {
                let key = t_v.get(0).unwrap();
                let v = t_v.get(1).unwrap();
                match key.as_str() {
                    "table" => orm_meta.table_name = v.to_string(),
                    "id" => orm_meta.id_name = v.to_string(),
                    "exclude" => {
                        orm_meta.exclude_field =
                            v.to_string().split("|").map(|x| x.to_string()).collect();
                    }
                    _ => {}
                }
            }
        } else {
            match index {
                0 => orm_meta.table_name = token.to_string(),
                1 => orm_meta.id_name = token.to_string(),
                2 => {
                    orm_meta.exclude_field = token
                        .to_string()
                        .split("|")
                        .map(|x| x.to_string())
                        .collect();
                }
                _ => {}
            }
        }
    }
    orm_meta
}

// 字符串转换
fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}

#[allow(dead_code)]
fn snake_to_camel(snake_string: String) -> String {
    case_style::CaseStyle::from_snakecase(snake_string)
        .to_camelcase()
        .to_string()
}

fn build_orm_script_method() -> Vec<TokenStream2> {
    let mut orm_script_method_vec = vec![];
    orm_script_method_vec.push(build_raw_query_script());
    orm_script_method_vec.push(build_raw_query_script_by_id());
    orm_script_method_vec.push(build_query_script());
    orm_script_method_vec.push(build_query_script_by_id());
    orm_script_method_vec.push(build_delete_script());
    orm_script_method_vec.push(build_delete_script_by_id());
    orm_script_method_vec.push(build_update_script());
    orm_script_method_vec.push(build_update_script_by_id());
    orm_script_method_vec.push(build_insert_script());
    orm_script_method_vec.push(build_update_status_script_by_id());
    orm_script_method_vec
}

fn build_raw_query_script() -> TokenStream2 {
    quote!(
        pub fn raw_query_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<String> = Self::get_orm_fields();
            let mut query_script = BmbpScriptSql::new();
            query_script.select_string_slice(orm_fields.as_slice());
            query_script.from(orm_table.as_str());
            query_script
        }
    )
}

fn build_query_script() -> TokenStream2 {
    quote!(
        pub fn query_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut query_script = BmbpScriptSql::new();
            for (raw_field, field) in orm_fields.as_slice() {
                query_script.select(format!("{} as {}", raw_field, field).as_str());
            }
            query_script.from(orm_table.as_str());
            query_script
        }
    )
}

fn build_raw_query_script_by_id() -> TokenStream2 {
    quote!(
        pub fn raw_query_script_by_id() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<String> = Self::get_orm_fields();
            let mut query_script = BmbpScriptSql::new();
            query_script.select_string_slice(orm_fields.as_slice());
            query_script.from(orm_table.as_str());
            query_script.filter("r_id = #{rId}");
            query_script
        }
    )
}

fn build_query_script_by_id() -> TokenStream2 {
    quote!(
        pub fn query_script_by_id() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut query_script = BmbpScriptSql::new();
            for (raw_field, field) in orm_fields.as_slice() {
                query_script.select(format!("{} as {}", raw_field, field).as_str());
            }
            query_script.from(orm_table.as_str());
            query_script.filter("r_id = #{rId}");
            query_script
        }
    )
}

fn build_delete_script() -> TokenStream2 {
    quote!(
        pub fn delete_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let mut delete_script = BmbpScriptSql::new();
            delete_script.delete(orm_table.as_str());
            delete_script
        }
    )
}

fn build_delete_script_by_id() -> TokenStream2 {
    quote!(
        pub fn delete_script_by_id() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let mut delete_script = BmbpScriptSql::new();
            delete_script.delete(orm_table.as_str());
            delete_script.filter("r_id = #{rId}");
            delete_script
        }
    )
}

fn build_insert_script() -> TokenStream2 {
    quote!(
        pub fn insert_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut insert_script = BmbpScriptSql::new();
            for (raw_field, field) in orm_fields.as_slice() {
                insert_script
                    .insert_value(raw_field.as_str(), format!("#{{{}}}", raw_field).as_str());
            }
            insert_script.insert_into(orm_table.as_str());
            insert_script
        }
    )
}

fn build_update_script() -> TokenStream2 {
    quote!(
        pub fn update_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut update_script = BmbpScriptSql::new();
            for (raw_field, field) in orm_fields.as_slice() {
                update_script.set_value(raw_field.as_str(), format!("#{{{}}}", raw_field).as_str());
            }
            update_script.update(orm_table.as_str());
            update_script
        }
    )
}

fn build_update_script_by_id() -> TokenStream2 {
    quote!(
        pub fn update_script_by_id() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut update_script = BmbpScriptSql::new();
            for (raw_field, field) in orm_fields.as_slice() {
                update_script.set_value(raw_field.as_str(), format!("#{}", field).as_str());
            }
            update_script.update(orm_table.as_str());
            update_script.filter("r_id = #{rId}");
            update_script
        }
    )
}

fn build_update_status_script_by_id() -> TokenStream2 {
    quote!(
        pub fn update_satus_script() -> BmbpScriptSql {
            let orm_table: String = Self::get_orm_table();
            let orm_fields: Vec<(String, String)> = Self::get_orm_field_alias();
            let mut update_script = BmbpScriptSql::new();
            update_script.set_value("r_status", "#{rStatus}");
            update_script.update(orm_table.as_str());
            update_script.filter("r_id = #{rId}");
            update_script
        }
    )
}
