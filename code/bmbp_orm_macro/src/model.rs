use proc_macro::TokenStream;

#[allow(unused)]
pub fn model(model_meta_token: TokenStream, model_struct_token: TokenStream) -> TokenStream {
    model_struct_token.into()
}
