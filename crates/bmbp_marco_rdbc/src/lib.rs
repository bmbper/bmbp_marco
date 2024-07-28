use proc_macro::TokenStream;
mod bean;
mod consts;
mod marco_rdbc_model;
mod meta;
mod model;
mod orm_row;
mod table;
mod table_orm;
mod table_rdbc;
mod table_tree;
mod table_tree_orm;
mod table_tree_rdbc;
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

/// #[table(table_name)]
#[proc_macro_attribute]
pub fn table(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table::macro_table(meta_token, struct_token)
}
/// #[table_rdbc(table_name)]
#[proc_macro_attribute]
pub fn table_rdbc(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_rdbc::marco_table_rdbc(meta_token, struct_token)
}
/// #[table_orm(table_name)]
#[proc_macro_attribute]
pub fn table_orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_orm::marco_table_orm(meta_token, struct_token)
}

/// #[table_tree(table_name,tree_prefix)]
#[proc_macro_attribute]
pub fn table_tree(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_tree::marco_table_tree(meta_token, struct_token)
}
/// #[table_tree_rdbc(table_name,tree_prefix)]
#[proc_macro_attribute]
pub fn table_tree_rdbc(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_tree_rdbc::marco_table_tree_rdbc(meta_token, struct_token)
}
/// #[table_tree_orm(table_name,tree_prefix)]
#[proc_macro_attribute]
pub fn table_tree_orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    table_tree_orm::marco_table_tree_orm(meta_token, struct_token)
}

#[proc_macro_attribute]
pub fn from_row(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    orm_row::marco_orm_row(meta_token, struct_token)
}
