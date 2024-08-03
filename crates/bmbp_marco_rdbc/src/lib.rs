use proc_macro::TokenStream;
use crate::old::marco_rdbc_model;

mod old;
mod meta;
mod orm_row;
mod table_bean;
mod table_bean_orm;
mod table_rdbc_bean;
mod table_tree_bean;
mod table_tree_bean_orm;
mod table_rdbc_tree_bean;
mod table_rdbc_bean_orm;
mod table_rdbc_tree_bean_orm;

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


#[proc_macro_attribute]
pub fn table_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean::marco_table_bean(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_bean_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean::marco_table_bean_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_tree_bean::marco_table_tree_bean(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_tree_bean_option(
    bean_meta_token: TokenStream,
    bean_struct_token: TokenStream,
) -> TokenStream {
    let token = table_tree_bean::marco_table_tree_bean_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_bean::marco_table_rdbc_bean(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_bean_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_bean::marco_table_rdbc_bean_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_tree_bean::marco_table_rdbc_tree_bean(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_tree_bean_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_tree_bean::marco_table_rdbc_tree_bean_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_bean_orm(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean_orm::marco_table_bean_orm(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_bean_orm_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean_orm::marco_table_bean_orm_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_tree_bean_orm(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_tree_bean_orm::marco_table_tree_bean_orm(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_tree_bean_orm_option(
    bean_meta_token: TokenStream,
    bean_struct_token: TokenStream,
) -> TokenStream {
    let token = table_tree_bean_orm::marco_table_tree_bean_orm_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_bean_orm(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_bean_orm::marco_table_rdbc_bean_orm(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_bean_orm_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_bean_orm::table_rdbc_bean_orm_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_tree_bean_orm(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_tree_bean_orm::table_rdbc_tree_bean_orm(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_rdbc_tree_bean_orm_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_rdbc_tree_bean_orm::table_rdbc_tree_bean_orm_option(bean_meta_token, bean_struct_token);
    println!("==>{}", token.to_string());
    token
}

