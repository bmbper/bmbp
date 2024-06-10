use proc_macro::TokenStream;
mod marco_bean;

#[proc_macro_attribute]
pub fn bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    marco_bean::marco_bean(bean_meta_token, bean_struct_token)
}
