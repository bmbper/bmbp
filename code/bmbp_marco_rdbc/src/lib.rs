use proc_macro::TokenStream;
mod consts;
mod marco_rdbc_model;
mod model;
mod table;
mod table_orm;
mod table_rdbc;
mod types;
mod utils;

/// #[marco_rdbc_model] 给Struct增加  公共属性, get_,get_mut_,set_方法, set_方法,rdbc_row转换方法
/// #[marco_rdbc_model]
/// #[marco_rdbc_model()]
/// #[rdbc_model(BMBP_RDBC_APP)]
/// #[rdbc_model(BMBP_RDBC_APP, menu)]
/// #[rdbc_model(table=BMBP_RDBC_APP, menu)]
/// #[rdbc_model(table=BMBP_RDBC_APP, tree=menu)]
/// #[rdbc_model(table=BMBP_RDBC_APP, menu)]
/// #[rdbc_model(table=BMBP_RDBC_APP, tree=menu)]
#[proc_macro_attribute]
pub fn rdbc_model(model_meta_token: TokenStream, tree_struct_token: TokenStream) -> TokenStream {
    marco_rdbc_model::rdbc_model(model_meta_token, tree_struct_token)
}

/// #[table(table_name)] 仅增加获取表名、获取列表、列枚举方法
/// ```rust
///     #[table(bmbp_table)]
///     pub struct User{
///         #[id]
///         id:  String,
///         #[column(name=user_name)]
///         name: String,
///         #[column(ignore=true)]
///         organ: Organ
///     }
/// ```
#[proc_macro_attribute]
pub fn table(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table::macro_table(meta_token, struct_token)
}
///
/// #[table(table_name)] 增加公共字段后 增加获取表名、获取列表、列枚举方法
/// ```rust
///     #[table_rdbc(bmbp_table)]
///     pub struct User{
///         #[id]
///         id:  String,
///         #[column(name=user_name)]
///         name: String,
///         #[column(ignore=true)]
///         organ: Organ
///     }
/// ```
#[proc_macro_attribute]
pub fn table_rdbc(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_rdbc::marco_table_rdbc(meta_token, struct_token)
}
///
/// #[table_orm(table_name)] 增加公共字段后 增加获取表名、获取列表、列枚举方法 ， 并实现和数据库的交互
/// ```rust
///     #[table_orm(bmbp_table)]
///     pub struct User{
///         #[id]
///         id:  String,
///         #[column(name=user_name)]
///         name: String,
///         #[column(ignore=true)]
///         organ: Organ
///     }
/// ```
#[proc_macro_attribute]
pub fn table_orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_orm::marco_table_orm(meta_token, struct_token)
}
