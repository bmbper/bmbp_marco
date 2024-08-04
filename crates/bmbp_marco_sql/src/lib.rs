use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rdbc_sql(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
   bean_struct_token
}

#[proc_macro_attribute]
pub fn rdbc_query(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}
#[proc_macro_attribute]
pub fn rdbc_insert(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}
#[proc_macro_attribute]
pub fn rdbc_insert_sensitive(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}
#[proc_macro_attribute]
pub fn rdbc_update(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}
#[proc_macro_attribute]
pub fn rdbc_update_sensitive(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}
#[proc_macro_attribute]
pub fn rdbc_delete(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean_struct_token
}