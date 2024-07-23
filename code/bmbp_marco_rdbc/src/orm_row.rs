use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_orm_row(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    println!("==============》marco_orm_row==============={}", "");
    // 获取结构体名称
    let parse_struct_token = struct_token.clone();
    let struct_input_token = parse_macro_input!(parse_struct_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    // 赋值语句集合
    let field_set_value_vec: Vec<TokenStream2> = vec![];
    let from_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::new();
                #(#field_set_value_vec)*
                model
            }
        }
    };

    let token_vec = vec![struct_token, from_token.into()];
    let token = TokenStream::from_iter(token_vec);
    println!("=======>{}", token.to_string());
    token
}
