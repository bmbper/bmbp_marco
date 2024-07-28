use crate::meta::RdbcOrmMeta;
use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) fn marco_table_tree_orm(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let parse_token = struct_token.clone();
    let orm_meta = parse_macro_input!(meta_token as RdbcOrmMeta);
    let struct_meta = parse_macro_input!(parse_token as syn::ItemStruct);
    let struct_ident = struct_meta.ident.clone();

    struct_token
}
