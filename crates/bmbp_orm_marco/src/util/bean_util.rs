use crate::meta::TableTreeMeta;
use bmbp_sql::RdbcColumnIdent;
use case_style::CaseStyle;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::__private::TokenStream2;
use syn::{parse_quote, Attribute, DeriveInput, Field, Generics, Type, TypePath};

pub fn base_table_field() -> Vec<Field> {
    vec![
        parse_quote!(pub data_id: String),
        parse_quote!(pub data_status: String),
        parse_quote!(pub data_flag: String),
        parse_quote!(pub data_level: String),
        parse_quote!(pub data_sort: u64),
        parse_quote!(pub data_create_time: String),
        parse_quote!(pub data_update_time: String),
        parse_quote!(pub data_create_user: String),
        parse_quote!(pub data_update_user: String),
        parse_quote!(pub data_owner_org: String),
        parse_quote!(pub data_sign: String),
    ]
}

pub fn base_tree_field(tree_prefix: String, children_ident: &Ident) -> Vec<Field> {
    let code = format_ident!("{}_code", tree_prefix.clone());
    let code_path = format_ident!("{}_code_path", tree_prefix.clone());
    let parent_code = format_ident!("{}_parent_code", tree_prefix.clone());
    let name = format_ident!("{}_name", tree_prefix.clone());
    let name_path = format_ident!("{}_name_path", tree_prefix.clone());
    let node_type = format_ident!("{}_type", tree_prefix.clone());
    let node_leaf = format_ident!("{}_leaf", tree_prefix.clone());
    let node_icon = format_ident!("{}_icon", tree_prefix.clone());
    let node_children = format_ident!("{}_children", tree_prefix.clone());
    let node_sort = format_ident!("{}_sort", tree_prefix.clone());
    vec![
        parse_quote!(pub #code: String),
        parse_quote!(pub #code_path: String),
        parse_quote!(pub #parent_code: String),
        parse_quote!(pub #name: String),
        parse_quote!(pub #name_path: String),
        parse_quote!(pub #node_type: String),
        parse_quote!(pub #node_leaf: u64),
        parse_quote!(pub #node_icon: String),
        parse_quote!(pub #node_sort: u64),
        parse_quote!(pub #node_children: Vec<#children_ident>),
    ]
}

pub fn base_tree_table_field(tree_prefix: String, children_ident: &Ident) -> Vec<Field> {
    let base_field = base_table_field();
    let tree_field = base_tree_field(tree_prefix, children_ident);
    let mut all_field = base_field;
    all_field.extend(tree_field);
    return all_field;
}

pub(crate) fn build_table_name(table_tree_meta: &TableTreeMeta, struct_ident: &Ident) -> String {
    match table_tree_meta.get_table() {
        Some(table) => {
            if table.is_empty() {
                CaseStyle::guess(struct_ident.to_string())
                    .unwrap()
                    .to_snakecase()
                    .to_uppercase()
            } else {
                table.to_string()
            }
        }
        None => CaseStyle::guess(struct_ident.to_string())
            .unwrap()
            .to_snakecase()
            .to_uppercase(),
    }
}
pub(crate) fn parse_struct_fields(struct_input: &DeriveInput) -> Vec<Field> {
    let mut field_vec = vec![];
    match &struct_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    field_vec.push(field.clone())
                }
            }
            syn::Fields::Unnamed(_) => {}
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}
pub(crate) fn merge_struct_fields(
    mut struct_fields: Vec<Field>,
    from_fields: &[Field],
) -> Vec<Field> {
    let field_name_set = parse_field_name_set(struct_fields.as_slice());
    for field in from_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if !field_name_set.contains(&field_name) {
            struct_fields.push(field.clone());
        }
    }
    struct_fields
}
pub(crate) fn parse_field_name_set(fields: &[Field]) -> HashSet<String> {
    let mut field_name_set = HashSet::new();
    for item in fields {
        if let Some(field_ident) = &item.ident {
            field_name_set.insert(field_ident.to_string());
        }
    }
    field_name_set
}

pub fn build_struct_token(
    struct_ident: &Ident,
    struct_attrs: &[Attribute],
    struct_generics: &Generics,
    struct_fields: &[Field],
) -> TokenStream {
    let generics_param_token = build_generics_param_token(struct_generics);
    let generics_where_token = build_generics_where_token(struct_generics);
    let struct_field_token = build_struct_field_token(struct_fields);
    let attrs_token = if struct_attrs.is_empty() {
        quote! {
            #[derive(Default, Debug, Clone, Serialize, Deserialize)]
            #[serde(default)]
        }
    } else {
        quote! {
            #(#struct_attrs)*
        }
    };
    let bean_token = quote! {
        #attrs_token
        pub struct #struct_ident  #generics_param_token #generics_where_token {
               #(#struct_field_token,)*
        }
    };
    bean_token.into()
}

pub fn build_struct_field_token(struct_fields: &[Field]) -> Vec<TokenStream> {
    let mut field_vec = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_attrs = &field.attrs;
        let visibility = &field.vis;
        field_vec.push(quote! {
            #(#field_attrs)*
            #visibility #field_ident: #field_type
        });
    }
    field_vec
}

fn build_generics_param_token(struct_generics: &Generics) -> TokenStream {
    let params_empty = struct_generics.params.is_empty();
    if params_empty {
        return quote! {};
    }
    let params = &struct_generics.params;
    quote! {
        <#params>
    }
}

fn build_generics_where_token(struct_generics: &Generics) -> TokenStream {
    if struct_generics.where_clause.is_none() {
        return quote! {};
    }
    let where_clause = &struct_generics.where_clause;
    quote! {
        #where_clause
    }
}

pub(crate) fn build_impl_table_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
    table_name: String,
) -> TokenStream {
    let column_ident = format_ident!("{}Column", struct_ident);
    let mut column_enum_vec = vec![];
    let mut column_enum_name_vec = vec![];
    for field in struct_fields {
        let field_name = field.ident.as_ref().unwrap().name();
        let enum_field_name = CaseStyle::guess(field_name.clone())
            .unwrap()
            .to_pascalcase();
        let enum_field_name_ident = format_ident!("{}", enum_field_name);
        column_enum_vec.push(format_ident!("{}", enum_field_name_ident.clone()));
        column_enum_name_vec.push(quote! {
            #column_ident::#enum_field_name_ident => #field_name.to_string()
        });
    }
    let column_enum_token = quote! {
        pub enum #column_ident {
            #(#column_enum_vec),*
        }
    };
    let impl_column_token = quote! {
        impl RdbcColumnIdent for #column_ident {
            fn name(&self) -> String {
                match self {
                  #(#column_enum_name_vec,)*
                }
            }
        }
    };
    let impl_table_token = quote! {
        impl RdbcTableIdent for #struct_ident {
            fn name() -> String {
                #table_name.to_string()
            }
            fn alias() -> String {
                "".to_string()
            }
            fn columns() -> Vec<impl RdbcColumnIdent> {
                vec![
                    #(#column_ident::#column_enum_vec),*
                ]
            }
            fn primary_key() -> impl RdbcColumnIdent {
                "data_id".to_string()
            }
            fn union_primary_key() -> Vec<impl RdbcColumnIdent> {
                vec!["".to_string()]
            }
        }
    };
    let column_impl_token = quote! {
        #column_enum_token
        #impl_column_token
        #impl_table_token
    };
    column_impl_token.into()
}
pub(crate) fn build_impl_tree_token(struct_ident: &Ident, tree_prefix: String) -> TokenStream {
    let tree_code = format_ident!("{}_code", tree_prefix);
    let parent_code = format_ident!("{}_parent_code", tree_prefix);
    let children = format_ident!("{}_children", tree_prefix);
    let tree_sort = format_ident!("{}_sort", tree_prefix);

    quote! {
        impl BmbpTree<#struct_ident> for #struct_ident {
            fn get_code(&self) -> &String {
                &self.#tree_code
            }
            fn set_code(&mut self, code: String) -> &mut Self {
                self.#tree_code = code;
                self
            }
            fn get_parent_code(&self) -> &String {
                &self.#parent_code
            }
            fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
                self.#parent_code = parent_code;
                self
            }
            fn get_children(&self) -> &[#struct_ident] {
                self.#children.as_slice()
            }
            fn get_children_mut(&mut self) -> &mut [#struct_ident] {
                self.#children.as_mut()
            }
            fn set_children(&mut self, children: Vec<#struct_ident>) -> &mut Self {
                self.#children = children;
                self
            }
            fn get_order(&self) -> u64 {
                self.#tree_sort.clone()
            }
        }
    }
}

pub(crate) fn build_impl_orm_row_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
) -> TokenStream {
    let mut field_set_value_vec: Vec<TokenStream2> = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let token = if field_has_option_type(&field.ty) {
            quote! {
                  if let Some(data) = row.get_data().get(#field_name) {
                      model.#field_ident = Some(data.into());
                  }
            }
        } else {
            quote! {
                  if let Some(data) = row.get_data().get(#field_name) {
                      model.#field_ident = data.into();
                  }
            }
        };
        field_set_value_vec.push(token);
    }
    // 赋值语句集合
    let orm_row_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::default();
                #(#field_set_value_vec)*
                model
            }
        }
    };
    orm_row_token
}

pub fn field_has_option_type(field_type: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 {
            if path.segments[0].ident.to_string() == "Option" {
                return true;
            }
        }
    }
    false
}

pub fn field_has_attrs_ident(field: &Field, attrs: &str) -> bool {
    for attr_item in field.attrs.iter() {
        return attr_item.path().is_ident(attrs);
    }
    false
}
