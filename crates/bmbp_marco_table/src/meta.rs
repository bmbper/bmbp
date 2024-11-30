use case_style::CaseStyle;
use syn::parse::Parse;
use syn::{Expr, Lit, Meta, MetaNameValue, Token};
#[derive(Debug, Default, Clone)]
pub struct TableMeta {
    table: Option<String>,
}
impl TableMeta {
    pub fn table(&self) -> String {
        self.table.as_ref().unwrap_or(&"".to_string()).clone()
    }
}
impl Parse for TableMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // NameValue的赋值函数
        let set_token_value = |token: &mut Option<String>, value: MetaNameValue| {
            if let Expr::Lit(lit) = value.value {
                if let Lit::Str(lit_str) = lit.lit {
                    *token = Some(lit_str.value());
                }
            } else {
                if let Expr::Path(path) = value.value {
                    if let Some(ident) = path.path.get_ident() {
                        *token = Some(ident.to_string());
                    }
                }
            }
        };
        if input.is_empty() {
            return Ok(TableMeta { table: None });
        }
        let mut table_name: Option<String> = None;
        let mut others = Vec::new();
        while !input.is_empty() {
            if let Ok(meta) = input.parse::<Meta>() {
                match meta {
                    Meta::NameValue(name_value) => {
                        if name_value.path.is_ident("table") {
                            set_token_value(&mut table_name, name_value)
                        }
                    }
                    Meta::Path(path) => {
                        if let Some(ident) = path.get_ident() {
                            others.push(ident.to_string());
                        }
                    }
                    Meta::List(_) => {}
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        for item in others {
            if table_name.is_none() {
                table_name = Some(item);
            }
        }
        if let Some(table) = table_name {
            let snake_table = CaseStyle::guess(table)
                .unwrap()
                .to_snakecase()
                .to_uppercase();
            table_name = Some(snake_table);
        }
        Ok(TableMeta { table: table_name })
    }
}
