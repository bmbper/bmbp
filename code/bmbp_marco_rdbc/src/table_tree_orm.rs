use proc_macro::TokenStream;

pub(crate) fn marco_table_tree_orm(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let parse_token = struct_token.clone();
    struct_token
}
