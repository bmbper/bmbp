use proc_macro::TokenStream;
mod bean;
mod rdbc_bean;
mod rdbc_record;
mod rdbc_tree_bean;
mod tree_bean;
mod util;

/// bean  增加get set 方法
#[proc_macro_attribute]
pub fn bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
/// bean 增加get set 方法
/// #[tree_bean(organ)]
///
#[proc_macro_attribute]
pub fn tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    tree_bean::marco_tree_bean(bean_meta_token, bean_struct_token)
}
/// rdbc_bean 增加基础字段 并设置get set 方法
#[proc_macro_attribute]
pub fn rdbc_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
/// rdbc_bean 增加基础字段 并设置get set 方法
#[proc_macro_attribute]
pub fn rdbc_tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
