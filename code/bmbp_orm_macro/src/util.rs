use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, IdentFragment, ToTokens};
use syn::{parse, parse_str, Data, DeriveInput, Field, Meta, Type, TypePath};

pub fn build(
    struct_name: &Ident,
    struct_field: Vec<TokenStream>,
    struct_method: Vec<TokenStream>,
) -> TokenStream {
    quote!(
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_name{
             #(#struct_field,)*
        }
        impl #struct_name{
            #(#struct_method)*
        }
        impl #struct_name{
          pub fn new()->#struct_name{
                #struct_name::default()
            }
        }
    )
}

pub fn parse_struct(
    derive_input: &DeriveInput,
) -> ((Vec<(String, String)>, Vec<TokenStream>, Vec<TokenStream>)) {
    let mut orm_struct_field = vec![];
    let mut orm_struct_field_token = vec![];
    let mut orm_struct_method_token = vec![];

    let orm_struct_ident_data = &derive_input.data;
    match orm_struct_ident_data {
        Data::Struct(orm_struct_data) => {
            let orm_struct_fields = &orm_struct_data.fields;
            for struct_field in orm_struct_fields {
                let struct_field_name = &struct_field.ident.as_ref().unwrap().to_string();
                let struct_field_type_name = &struct_field.ty.to_token_stream().to_string();
                if !is_orm_struct_field_has_skip_meta(struct_field) {
                    orm_struct_field
                        .push((struct_field_name.clone(), struct_field_type_name.clone()));
                }
                let orm_struct_field_ident = struct_field.ident.as_ref().unwrap();
                let orm_struct_field_type = &struct_field.ty;
                // 增加结构字段
                orm_struct_field_token.push(quote!(
                    #orm_struct_field_ident : #orm_struct_field_type
                ));
                let field_method_token =
                    build_token_with_name_type(&orm_struct_field_ident, orm_struct_field_type);
                orm_struct_method_token.push(field_method_token.to_token_stream());
            }
        }
        _ => {}
    };
    (
        orm_struct_field,
        orm_struct_field_token,
        orm_struct_method_token,
    )
}

fn is_orm_struct_field_has_skip_meta(field: &Field) -> bool {
    // 从属性里面提取宏
    let field_attrs = field.attrs.as_slice();
    if field_attrs.is_empty() {
        return false;
    };
    let mut has_skip_marco = false;
    for attr in field_attrs {
        let meta = &attr.meta;
        match meta {
            Meta::Path(mv) => {
                let temp = mv.to_token_stream().to_string();
                if "skip".eq(temp.as_str()) {
                    has_skip_marco = true;
                    break;
                }
            }
            Meta::List(v) => {
                println!("meta2:{:#?}", v.to_token_stream());
            }
            Meta::NameValue(v) => {
                println!("meta3:{:#?}", v.to_token_stream());
            }
        }
    }
    has_skip_marco
}

pub fn build_token_by_field(
    field_slice: &[(String, String)],
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut field_token_vec = vec![];
    let mut method_token_vec = vec![];
    for field_name in field_slice {
        let field_name_ident = format_ident!("{}", field_name.0.clone());
        let field_type_ident = format_ident!("{}", field_name.1.clone());
        let field_token = quote!(
           #field_name_ident : #field_type_ident
        );
        field_token_vec.push(field_token);

        let token = build_token_with_name_type(&field_name_ident, &field_type_ident);
        method_token_vec.push(token);
    }
    (field_token_vec, method_token_vec)
}

fn build_token_with_name_type<T>(field_name_ident: &Ident, field_type_ident: &T) -> TokenStream
where
    T: ToTokens,
{
    let set_method_ident = format_ident!("set_{}", field_name_ident);
    let get_method_ident = format_ident!("get_{}", field_name_ident);
    let get_mut_method_ident = format_ident!("get_mut_{}", field_name_ident);
    quote!(
        pub fn #set_method_ident(&mut self,v : #field_type_ident)->&mut Self{
            self.#field_name_ident = v;
            self
        }
        pub fn #get_method_ident(&self)->&#field_type_ident{
            &self.#field_name_ident
        }
        pub fn #get_mut_method_ident(&mut self)->&mut #field_type_ident{
            &mut self.#field_name_ident
        }
    )
}
