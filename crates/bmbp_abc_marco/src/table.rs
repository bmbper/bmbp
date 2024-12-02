use crate::meta::TableMeta;
use case_style::CaseStyle;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Field, Type};

pub(crate) fn generate_table_code(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let table_meta = parse_macro_input!(meta_token as TableMeta);
    let struct_token_clone = struct_token.clone();
    let struct_input = parse_macro_input!(struct_token_clone as DeriveInput);
    let struct_name = &struct_input.ident;

    // Generate column enum first
    let column_enum_token = generate_column_enum(&struct_input).into();

    // Generate table struct after column enum
    let table_struct_token = generate_table_struct(table_meta, struct_name).into();

    // Combine all tokens
    let combined_tokens = vec![struct_token, column_enum_token, table_struct_token];
    combined_tokens.into_iter().collect()
}

fn generate_column_enum(struct_input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = extract_struct_fields(struct_input);
    let column_enum_name = format_ident!("{}Column", struct_input.ident);

    let mut enum_variants = vec![];

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = &field.ty;

        if let Type::Path(type_path) = field_type {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();

            if type_name == "BmbpTree" {
                enum_variants.extend(generate_bmbp_tree_variants());
            } else if type_name == "BmbpBase" {
                enum_variants.extend(generate_bmbp_base_variants());
            } else {
                enum_variants.push(format_ident!(
                    "{}",
                    CaseStyle::guess(field_name).unwrap().to_pascalcase()
                ));
            }
        } else {
            enum_variants.push(format_ident!(
                "{}",
                CaseStyle::guess(field_name).unwrap().to_pascalcase()
            ));
        }
    }

    let to_string_match_arms = enum_variants.iter().map(|variant| {
        let enum_value = CaseStyle::guess(variant.to_string())
            .unwrap()
            .to_snakecase()
            .to_uppercase();
        quote! {
            #column_enum_name::#variant => #enum_value.to_string()
        }
    });

    let all_columns = enum_variants
        .iter()
        .map(|variant| quote! { #column_enum_name::#variant });

    quote! {
        pub enum #column_enum_name {
            #(#enum_variants),*
        }

        impl #column_enum_name {
            pub fn all_columns() -> Vec<#column_enum_name> {
                vec![#(#all_columns),*]
            }
        }

        impl ToString for #column_enum_name {
            fn to_string(&self) -> String {
                match self {
                    #(#to_string_match_arms),*
                }
            }
        }
    }
}

fn generate_table_struct(table_meta: TableMeta, struct_name: &Ident) -> proc_macro2::TokenStream {
    let table_name_snake_case = CaseStyle::guess(struct_name.to_string())
        .unwrap()
        .to_snakecase()
        .to_uppercase();

    let table_name = if table_meta.table().is_empty() {
        table_name_snake_case.clone()
    } else {
        table_meta.table()
    };

    let table_struct_name = format_ident!("{}Table", struct_name);
    let column_enum_name = format_ident!("{}Column", struct_name);

    quote! {
        pub struct #table_struct_name;

        impl #table_struct_name {
            pub fn name() -> String {
                #table_name.to_string()
            }

            pub fn columns() -> Vec<#column_enum_name> {
                #column_enum_name::all_columns()
            }
        }
    }
}

fn generate_bmbp_base_variants() -> Vec<Ident> {
    vec![
        format_ident!("DataId"),
        format_ident!("DataStatus"),
        format_ident!("DataLevel"),
        format_ident!("DataFlag"),
        format_ident!("DataSign"),
        format_ident!("DataOwnerOrgCode"),
        format_ident!("DataCreateTime"),
        format_ident!("DataCreateUser"),
        format_ident!("DataUpdateTime"),
        format_ident!("DataUpdateUser"),
    ]
}

fn generate_bmbp_tree_variants() -> Vec<Ident> {
    let mut variants = vec![
        format_ident!("ParentNodeCode"),
        format_ident!("NodeCode"),
        format_ident!("NodeCodePath"),
        format_ident!("NodeName"),
        format_ident!("NodeNamePath"),
        format_ident!("NodeDesc"),
        format_ident!("NodeGrade"),
        format_ident!("NodeType"),
        format_ident!("NodeOrder"),
    ];
    variants.extend(generate_bmbp_base_variants());
    variants
}

fn extract_struct_fields(struct_input: &DeriveInput) -> Vec<Field> {
    if let syn::Data::Struct(data_struct) = &struct_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().cloned().collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}
