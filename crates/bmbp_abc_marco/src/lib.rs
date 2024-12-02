mod meta;
mod table;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn table(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table::generate_table_code(meta_token, struct_token)
}
