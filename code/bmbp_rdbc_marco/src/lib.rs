use proc_macro::TokenStream;

mod consts;
mod rdbc_model;
mod rdbc_tree_model;
mod types;
mod utils;

/// #[rdbc_model] 给Struct增加  公共属性, get_,get_mut_,set_方法, set_方法,rdbc_row转换方法
///  #[rdbc_model(RBDC_MODEL)]
#[proc_macro_attribute]
pub fn rdbc_model(model_meta_token: TokenStream, tree_struct_token: TokenStream) -> TokenStream {
    rdbc_model::rdbc_model(model_meta_token, tree_struct_token)
}
/// #[rdbc_tree_model] 给Struct增加  公共属性, 树属性, ,get_,get_mut_,set_方法, set_方法,rdbc_role转换方法,treeTrait方法
#[proc_macro_attribute]
pub fn rdbc_tree_model(
    tree_meta_token: TokenStream,
    tree_struct_token: TokenStream,
) -> TokenStream {
    tree_struct_token
}
