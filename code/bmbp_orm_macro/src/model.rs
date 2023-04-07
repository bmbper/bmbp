use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Field, Ident, ItemStruct, Meta};

pub fn model(_: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    // 处理Struct定义的属性
    let model_struct_ident = parse_macro_input!(model_struct_token as DeriveInput);
    let mut model_struct_field_token = vec![];
    let mut model_struct_method_token = vec![];

    let model_struct_ident_data = &model_struct_ident.data;
    match model_struct_ident_data {
        Data::Struct(model_struct_data) => {
            let model_struct_fields = &model_struct_data.fields;
            for struct_field in model_struct_fields {
                let struct_field_name = &struct_field.ident.as_ref().unwrap().to_string();
                let model_struct_field_ident = struct_field.ident.as_ref().unwrap();
                let model_struct_field_type = &struct_field.ty;

                // 增加结构字段
                model_struct_field_token.push(quote!(
                    #model_struct_field_ident : #model_struct_field_type
                ));

                let set_field_method_ident = format_ident!("set_{}", model_struct_field_ident);
                let get_field_method_ident = format_ident!("get_{}", model_struct_field_ident);
                let get_mut_field_ident = format_ident!("get_mut_{}", model_struct_field_ident);
                let field_method_token = quote!(
                    pub fn #set_field_method_ident(&mut self,v : #model_struct_field_type)->&mut Self{
                        self.#model_struct_field_ident = v;
                        self
                    }
                    pub fn #get_field_method_ident(&mut self)->& #model_struct_field_type{
                        &self.#model_struct_field_ident
                    }
                    pub fn #get_mut_field_ident(&mut self)->& mut #model_struct_field_type{
                        &mut self.#model_struct_field_ident
                    }
                );
                model_struct_method_token.push(field_method_token.to_token_stream());
            }
        }
        _ => {}
    }

    let struct_name_ident = &model_struct_ident.ident;
    let new_struct_token = quote!(
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_name_ident{
             #(#model_struct_field_token,)*
        }
        impl #struct_name_ident{
            #(#model_struct_method_token)*
        }
    );
    new_struct_token.into()
}
