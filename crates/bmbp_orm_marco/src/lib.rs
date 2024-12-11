use proc_macro::TokenStream;

mod meta;
mod table;
mod tree_table;
mod util;

#[proc_macro_attribute]
pub fn table(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table::generate_table_bean(bean_meta_token, bean_struct_token);
    token
}

#[proc_macro_attribute]
pub fn tree_table(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = tree_table::generate_tree_table(bean_meta_token, bean_struct_token);
    token
}
