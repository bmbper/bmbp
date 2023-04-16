use proc_macro::TokenStream;

pub fn validator(_: TokenStream, orm_struct_token: TokenStream) -> TokenStream {
    orm_struct_token.into()
}
