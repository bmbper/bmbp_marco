use proc_macro::TokenStream;

pub(crate) fn macro_table(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    struct_token
}
