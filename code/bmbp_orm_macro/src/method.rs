use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::{parse_macro_input, Field};

use crate::util;

pub fn method(_: TokenStream, struct_token: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(struct_token as DeriveInput);
    // 类字段
    let struct_field: Vec<(String, Field)> = util::parse_struct_field(&derive_input);
    let mut struct_method_vec = vec![];
    for (_, field) in struct_field {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        let set_method = format_ident!("set_{}", field_name);
        let get_method = format_ident!("get_{}", field_name);
        let get_mut_method = format_ident!("get_mut_{}", field_name);

        let field_method = quote!(
            pub fn  #set_method(&mut self,#field_name:#field_type)->&mut Self{
                self.#field_name = #field_name;
                self
            }
            pub fn  #get_mut_method(&mut self)->&mut #field_type{
               &mut self.#field_name
            }
            pub fn  #get_method(&self)->&#field_type{
               &self.#field_name
            }
        );
        struct_method_vec.push(field_method);
    }
    let struct_ident = &derive_input.ident;
    let impl_struct_token = quote!(
        impl #struct_ident{
            #(#struct_method_vec)*
        }
    );
    let final_struct = quote!(
        #derive_input
        #impl_struct_token
    );
    final_struct.into()
}
