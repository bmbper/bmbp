use proc_macro::TokenStream;

pub fn validator(valid_meta_token: TokenStream, orm_struct_token: TokenStream) -> TokenStream {
    orm_struct_token.into()
}
