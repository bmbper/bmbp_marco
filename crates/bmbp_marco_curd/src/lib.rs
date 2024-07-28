use proc_macro::TokenStream;
mod curd;
mod curd_dao;
mod curd_router;
mod curd_service;
mod curd_valid;
#[proc_macro_attribute]
pub fn curd(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    curd::marco_curd(meta_token, struct_token)
}

#[proc_macro_attribute]
pub fn curd_dao(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    curd_dao::marco_curd_dao(meta_token, struct_token)
}

#[proc_macro_attribute]
pub fn curd_service(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    curd_service::marco_curd_service(meta_token, struct_token)
}

#[proc_macro_attribute]
pub fn curd_router(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    curd_router::marco_curd_router(meta_token, struct_token)
}
