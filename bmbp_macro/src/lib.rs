extern crate proc_macro;

use proc_macro::TokenStream;

use quote::format_ident;
use quote::quote;
use syn::{DeriveInput, Fields};

#[proc_macro_derive(Orm)]
pub fn bmbp_orm(input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(input).unwrap();
    impl_bmbp_orm_macro(&mut ast)
}

fn impl_bmbp_orm_macro(ast: &mut DeriveInput) -> TokenStream {
    // 获取结构体字段
    let ast_data = &ast.data;
    let mut vo_field_vec = vec![];
    if let syn::Data::Struct(v) = ast_data {
        let fields = &v.fields;
        match fields {
            Fields::Named(named_fields) => {
                for name_field in &named_fields.named {
                    let named_field_ident = name_field.ident.as_ref().unwrap();
                    // 属性名称
                    vo_field_vec.push(named_field_ident.to_string());
                    let _ty = &name_field.ty;
                    let attrs = name_field.attrs.as_slice();
                    for _attr in attrs {}
                }
            }
            _ => {}
        }
    }
    let povo_name = &ast.ident;
    let orm_token = quote! {
        impl #povo_name {
            fn orm_fields()->Vec<String> {
               vec![#(#vo_field_vec.to_string()),*]
            }
        }
    };
    orm_token.into()
}

#[proc_macro_derive(PoVo)]
pub fn bmbp_povo(input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(input).unwrap();
    impl_bmbp_povo_macro(&mut ast)
}

fn impl_bmbp_povo_macro(ast: &mut DeriveInput) -> TokenStream {
    // 获取结构体字段
    let ast_data = &ast.data;
    let mut fn_token_vec = vec![];
    if let syn::Data::Struct(v) = ast_data {
        let fields = &v.fields;
        match fields {
            Fields::Named(named_fields) => {
                for name_field in &named_fields.named {
                    let named_field_ident = name_field.ident.as_ref().unwrap();
                    // 属性名称
                    let ty = &name_field.ty;
                    let get_fn_name = format_ident!("get_{}", named_field_ident);
                    let get_mut_fn_name = format_ident!("get_mut_{}", named_field_ident);
                    let set_fn_name = format_ident!("set_{}", named_field_ident);

                    let fn_token = quote! {
                        pub fn #get_fn_name(&self)-> &#ty{
                            &self.#named_field_ident
                        }

                        pub fn #get_mut_fn_name(&mut self)-> &mut #ty{
                           &mut self.#named_field_ident
                        }

                        pub fn #set_fn_name(&mut self,#named_field_ident:#ty)->&mut Self{
                            self.#named_field_ident = #named_field_ident;
                            self
                        }
                    };
                    fn_token_vec.push(fn_token);
                }
            }
            _ => {}
        }
    }
    let povo_name = &ast.ident;
    let orm_token = quote! {
        impl #povo_name {
            #(#fn_token_vec)*
        }
    };
    orm_token.into()
}
