use proc_macro::TokenStream;

#[allow(unused)]
pub fn tree_model(tree_meta_token: TokenStream, tree_struct_token: TokenStream) -> TokenStream {
    tree_struct_token.into()
}
