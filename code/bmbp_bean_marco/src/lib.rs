use proc_macro::TokenStream;
mod bean;
mod bmbp_curd;
mod rdbc_bean;
mod rdbc_record;
mod rdbc_tree_bean;
mod tree_bean;

/// bean 增加get set 方法
#[proc_macro_attribute]
pub fn bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
/// bean 增加get set 方法
#[proc_macro_attribute]
pub fn tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
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
/// rdbc_table 设置表模型
#[proc_macro_attribute]
pub fn rdbc_table(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
/// rdbc_record 数据库记录
#[proc_macro_attribute]
pub fn rdbc_record(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
/// bmbp_curd 增加 增、删、改、查方法
#[proc_macro_attribute]
pub fn bmbp_curd(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
