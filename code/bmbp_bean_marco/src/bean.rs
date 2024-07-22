use crate::util::parse_struct_fields;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Field};

pub(crate) fn marco_bean(_: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_fields = parse_struct_fields(&struct_input_token);
    let struct_field_token = build_struct_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_fields.as_slice());
    let bean_token = quote! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_ident {
               #(#struct_field_token,)*
        }
        impl #struct_ident {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
        }
    };
    bean_token.into()
}
fn build_struct_field_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut field_vec = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        field_vec.push(quote! {
             #field_ident: #field_type
        });
    }
    field_vec
}

fn build_struct_props_method_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let set_method_name = format_ident!("set_{}", field_name);
        let get_method_name = format_ident!("get_{}", field_name);
        let get_mut_method_name = format_ident!("get_mut_{}", field_name);
        let field_type = &item.ty;
        let method_token = quote! {
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
        };
        method_vec.push(method_token);
    }
    method_vec
}
