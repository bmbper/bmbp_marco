use proc_macro::TokenStream;


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
mod util;

#[proc_macro_attribute]
pub fn table_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean::marco_table_bean(bean_meta_token, bean_struct_token);
    println!("table_bean==>{}", token.to_string());
    token
}

#[proc_macro_attribute]
pub fn table_bean_option(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table_bean::marco_table_bean_option(bean_meta_token, bean_struct_token);
    println!("table_bean_option==>{}", token.to_string());
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

