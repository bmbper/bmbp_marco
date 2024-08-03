use proc_macro::TokenStream;

pub(crate) fn marco_table_orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    struct_token
}

pub(crate) fn marco_table_bean_orm(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {struct_token}

pub(crate) fn marco_table_bean_orm_option(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {struct_token}