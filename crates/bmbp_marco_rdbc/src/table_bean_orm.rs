use crate::meta::RdbcTableTreeMeta;
use crate::util::{build_impl_orm_row_token, build_struct_table_token, build_table_name};
use bmbp_marco_util::{
    build_struct_field_token, build_struct_option_field_token,
    build_struct_option_props_method_token, build_struct_props_method_token, build_struct_token,
    parse_struct_fields,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub(crate) fn marco_table_bean_orm(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let rdbc_meta = parse_macro_input!(meta_token as RdbcTableTreeMeta);
    let temp_struct_token = struct_token.clone();
    let struct_input = parse_macro_input!(temp_struct_token as syn::DeriveInput);
    let struct_ident = &struct_input.ident;
    let struct_generics = &struct_input.generics;
    let table_name = build_table_name(&rdbc_meta, struct_ident);
    let struct_attrs = &struct_input.attrs.as_slice();
    let struct_fields = parse_struct_fields(&struct_input);
    let struct_field_token = build_struct_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_fields.as_slice());
    let token = build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    );
    let table_struct_token =
        build_struct_table_token(struct_ident, &table_name, struct_fields.as_slice());
    let table_orm_token = build_impl_orm_row_token(struct_ident, struct_fields.as_slice());
    let tokens = quote! {
         #token
         #table_struct_token
        # table_orm_token
    };
    tokens.into()
}

pub(crate) fn marco_table_bean_orm_option(
    meta_token: TokenStream,
    struct_token: TokenStream,
) -> TokenStream {
    let rdbc_meta = parse_macro_input!(meta_token as RdbcTableTreeMeta);
    let temp_struct_token = struct_token.clone();
    let struct_input = parse_macro_input!(temp_struct_token as syn::DeriveInput);
    let struct_ident = &struct_input.ident;
    let struct_generics = &struct_input.generics;
    let table_name = build_table_name(&rdbc_meta, struct_ident);
    let struct_attrs = &struct_input.attrs.as_slice();
    let struct_fields = parse_struct_fields(&struct_input);
    let struct_field_token = build_struct_option_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_option_props_method_token(struct_fields.as_slice());
    let token = build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    );
    let table_struct_token =
        build_struct_table_token(struct_ident, &table_name, struct_fields.as_slice());
    let table_orm_token = build_impl_orm_row_token(struct_ident, struct_fields.as_slice());
    let tokens = quote! {
         #token
         #table_struct_token
        # table_orm_token
    };
    tokens.into()
}
