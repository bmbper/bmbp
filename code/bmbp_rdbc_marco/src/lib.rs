use proc_macro::TokenStream;
mod consts;
mod marco_rdbc_model;
mod model;
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

///
/// ```rust
///     #[rdbc_table]
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
pub fn rdbc_table(model_meta_token: TokenStream, tree_struct_token: TokenStream) -> TokenStream {
    tree_struct_token
}

/// 数据库记录
/// ```rust
/// use bmbp_rdbc_marco::orm_record;
/// #[orm_record(table_name)]
/// pub struct User{
///     #[id]
///     id:  String,
///     name: String,
///     #[skip]
///     organ: Organ
/// }
///
/// impl RdbcActiveModel<User> for User{
///     fn get_table_name() -> String {
///         "user".to_string()
///     }
/// }
///
/// ```
///
#[proc_macro_attribute]
pub fn orm_record(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    struct_token
}
