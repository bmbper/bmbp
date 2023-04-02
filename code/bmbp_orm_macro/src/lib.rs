use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Field, Ident, ItemStruct, Meta};

#[derive(Debug)]
struct OrmToken {
    table_name: String,
    id_name: String,
}

#[proc_macro_attribute]
pub fn orm(orm_args: TokenStream, struct_token: TokenStream) -> TokenStream {
    let mut orm_token = get_orm_token(&orm_args);
    let st = parse_macro_input!(struct_token as DeriveInput);
    let st_name = &st.ident.to_string().replace("\"", "");
    if orm_token.table_name.is_empty() {
        orm_token.table_name = camel_to_snake(st_name.clone());
    }

    // 用于生成SQL语句
    let mut st_field_names = vec![];
    let mut struct_method = vec![];
    let mut struct_fields = vec![];
    let st_data_enum = &st.data;
    match st_data_enum {
        Data::Struct(st_data) => {
            let st_fields = &st_data.fields;
            for field in st_fields {
                let field_name = &field.ident.as_ref().unwrap().to_string();
                if !struct_field_has_skip_marco(field) {
                    st_field_names.push(field_name.clone());
                }
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                struct_fields.push(quote!(
                    #ident : #ty ,
                ));

                let set_ident = format_ident!("set_{}", ident);
                let get_ident = format_ident!("get_{}", ident);
                let temp = quote!(
                    pub fn #set_ident(&mut self,v : #ty)->&mut Self{
                        self.#ident = v;
                        self
                    }
                    pub fn #get_ident(&mut self)->& #ty{
                        &self.#ident
                    }
                );
                struct_method.push(temp.to_token_stream());
            }
        }
        _ => {}
    }

    let orm_field_method = quote!(
        pub fn get_orm_fields(&self) -> Vec<String> {
            vec![
                #(#st_field_names.to_string()),*
            ]
        }
    );
    struct_method.push(orm_field_method);

    let struct_ident = &st.ident;
    let struct_quote = quote!(
        pub struct #struct_ident{
            #(#struct_fields)*
        }

        impl #struct_ident{
            #(#struct_method)*
        }
    );
    struct_quote.into()
}

fn get_orm_token(token_stream: &TokenStream) -> OrmToken {
    let mut orm_token = OrmToken {
        table_name: "".to_string(),
        id_name: "".to_string(),
    };
    let mut token_tree = vec![];
    for item in token_stream.clone().into_iter() {
        token_tree.push(item);
    }
    match token_tree.len() {
        0 => {}
        1 => set_orm_table_token(&mut orm_token, &token_tree[0]),
        3 => set_orm_token(
            &mut orm_token,
            &token_tree[0],
            &token_tree[1],
            &token_tree[2],
        ),
        7 => {
            let split = get_token_tree_node_name(&token_tree[3]);
            if "," == split.as_str() {
                set_orm_token(
                    &mut orm_token,
                    &token_tree[0],
                    &token_tree[1],
                    &token_tree[2],
                );
                set_orm_token(
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

fn set_orm_table_token(orm: &mut OrmToken, token: &TokenTree) {
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

fn set_orm_token(orm: &mut OrmToken, left: &TokenTree, split: &TokenTree, right: &TokenTree) {
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

fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}

fn struct_field_has_skip_marco(field: &Field) -> bool {
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
