use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident};

use crate::util;

pub fn base(_: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    let model_struct = parse_macro_input!(model_struct_token as DeriveInput);
    let struct_name_ident: &Ident = &model_struct.ident;

    // 公共字段
    let base_field = util::build_base_field_token_stream();

    let mut base_method_vec = vec![];
    for (field_name, _) in base_field.as_slice() {
        let raw_field_name = field_name.to_string().replace("\"", "");
        let field_ident = format_ident!("{}", raw_field_name);
        let get_field_method_ident = format_ident!("get_base_{}", raw_field_name);
        let set_field_method_ident = format_ident!("set_base_{}", raw_field_name);
        let method_token = quote!(
            fn #get_field_method_ident(&self)->&String{
                &self.#field_ident
            }
            fn #set_field_method_ident(&mut self,#field_ident:String)->&mut Self{
                self.#field_ident = #field_ident;
                self
            }
        );
        base_method_vec.push(method_token);
    }

    let impl_base_model_trait = quote!(
        impl BmbpBaseModelTrait for #struct_name_ident{
             #(#base_method_vec)*
        }
    );

    // 自定义字段
    let old_struct_field = util::parse_struct_field(&model_struct);

    // 合并字段
    let struct_merge_filed =
        util::merge_field_token_stream_from_field(base_field, old_struct_field);

    let struct_atts = &model_struct.attrs;
    let final_struct = quote!(
        #(#struct_atts)*
        pub struct #struct_name_ident{
           #(#struct_merge_filed),*
        }
        #impl_base_model_trait
    );
    final_struct.into()
}
