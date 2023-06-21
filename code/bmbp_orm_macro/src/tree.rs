use proc_macro::{TokenStream, TokenTree};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::{parse_macro_input, Field};

use crate::util;

pub fn tree(tree_meta_token: TokenStream, tree_struct_token: TokenStream) -> TokenStream {
    let tree_prefix = get_tree_meta(tree_meta_token);
    let tree_derive_input = parse_macro_input!(tree_struct_token as DeriveInput);

    // 类名称
    let tree_struct_name = &tree_derive_input.ident.to_string().replace("\"", "");

    // 类树接口实现
    let tree_node_trait = build_tree_node_trait(tree_prefix.clone(), tree_struct_name.clone());

    // 类增加树型结构字段
    let tree_field: Vec<(String, TokenStream2)> =
        util::build_tree_field_token_stream(tree_prefix.clone(), tree_struct_name.clone());
    // 类本身定义字段
    let struct_field: Vec<(String, Field)> = util::parse_struct_field(&tree_derive_input);

    // 类去重字段，以#[tree]定义为主
    let struct_merge_filed: Vec<TokenStream2> =
        util::merge_field_token_stream_from_field(tree_field, struct_field);
    // 获取类的宏修钸
    let struct_atts = &tree_derive_input.attrs;
    let struct_name_ident = format_ident!("{}", tree_struct_name);
    let final_struct = quote!(
        #(#struct_atts)*
        pub struct #struct_name_ident{
           #(#struct_merge_filed),*
        }
    );
    let token_vec = vec![final_struct, tree_node_trait];
    let real_token = quote!(#(#token_vec)*);
    real_token.into()
}

pub fn get_tree_meta(tree_meta_token: TokenStream) -> String {
    let mut token_tree = vec![];
    for item in tree_meta_token.clone().into_iter() {
        token_tree.push(item);
    }
    if token_tree.is_empty() {
        return "".to_string();
    }
    let token = token_tree.get(0).unwrap();
    let tree_prefix = match token {
        TokenTree::Group(_) => "".to_string(),
        TokenTree::Ident(id) => id.to_string(),
        TokenTree::Punct(_) => "".to_string(),
        TokenTree::Literal(it) => it.to_string().replace("\"", ""),
    };
    tree_prefix
}

pub fn build_tree_node_trait(tree_prefix: String, tree_struct_name: String) -> TokenStream2 {
    // tree_field
    let tree_id = format_ident!("{}_code", tree_prefix);
    let tree_parent_id = format_ident!("{}_parent_code", tree_prefix);
    let tree_data_id = format_ident!("{}_data_id", tree_prefix);
    let tree_title = format_ident!("{}_title", tree_prefix);
    let tree_children = format_ident!("{}_children", tree_prefix);
    let tree_id_path = format_ident!("{}_code_path", tree_prefix);
    let tree_title_path = format_ident!("{}_title_path", tree_prefix);
    let tree_struct = format_ident!("{}", tree_struct_name);

    let tree_trait = quote!(
        // impl BmbpTree<#tree_struct> for  #tree_struct {
        //     fn get_tree_code(&self) -> Option<&String>{
        //         &self.#tree_id
        //     }

        //     fn get_tree_parent_code(&self) -> Option<&String> {
        //        &self.#tree_parent_id
        //     }

        //     fn get_tree_data_id(&self) -> &String {
        //         &self.#tree_data_id
        //     }

        //     fn get_tree_title(&self) -> &String {
        //          &self.#tree_title
        //     }

        //     fn get_tree_id_path(&self) -> &String {
        //         &self.#tree_id_path
        //     }

        //     fn get_tree_title_path(&self) -> &String {
        //         &self.#tree_title_path
        //     }

        //     fn get_tree_children(&self) -> &[#tree_struct] {
        //        self.#tree_children.as_slice()
        //     }

        //     fn get_mut_tree_children(&mut self) -> &mut Vec<#tree_struct> {
        //         &mut self.#tree_children
        //     }

        //     fn set_tree_id(&mut self, #tree_id: String) -> &mut Self {
        //         self.#tree_id = #tree_id;
        //         self
        //     }

        //     fn set_tree_parent_id(&mut self, #tree_parent_id: String) -> &mut Self {
        //         self.#tree_parent_id = #tree_parent_id;
        //         self
        //     }

        //     fn set_tree_data_id(&mut self, #tree_data_id: String) -> &mut Self {
        //         self.#tree_data_id = #tree_data_id;
        //         self
        //     }

        //     fn set_tree_title(&mut self, #tree_title: String) -> &mut Self {
        //         self.#tree_title = #tree_title;
        //         self
        //     }

        //     fn set_tree_id_path(&mut self, #tree_id_path: String) -> &mut Self {
        //         self.#tree_id_path = #tree_id_path;
        //         self
        //     }

        //     fn set_tree_title_path(&mut self, #tree_title_path: String) -> &mut Self {
        //         self.#tree_title_path = #tree_title_path;
        //         self
        //     }

        //     fn set_tree_children(&mut self, #tree_children: Vec<#tree_struct>) -> &mut Self {
        //         self.#tree_children = #tree_children;
        //         self
        //     }
        // }

    );

    tree_trait
}
