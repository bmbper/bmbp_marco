use bmbp_marco_util::util::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_tree_bean(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_attrs = struct_input_token.attrs.as_slice();
    let struct_generics= &struct_input_token.generics;
    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token.into());
    let tree_field_name = build_tree_field_name(tree_prefix);
    let mut tree_field = build_tree_field(tree_field_name.as_slice(), &struct_ident);
    let mut struct_fields = parse_struct_fields(&struct_input_token);
    struct_fields = merge_struct_fields(struct_fields,tree_field.as_slice());

    let struct_field_token = build_struct_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_fields.as_slice());
    build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    )
    .into()
}
pub(crate) fn marco_option_tree_bean(
    meta_token: TokenStream,
    model_token: TokenStream,
) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_generics= &struct_input_token.generics;
    let struct_attrs = struct_input_token.attrs.as_slice();
    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token.into());
    let tree_field_name = build_tree_field_name(tree_prefix);
    let mut tree_field = build_tree_field(tree_field_name.as_slice(), &struct_ident);
    let mut struct_fields = parse_struct_fields(&struct_input_token);
    struct_fields = merge_struct_fields(struct_fields,tree_field.as_slice());

    let struct_field_token = build_struct_option_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_option_props_method_token(struct_fields.as_slice());
    build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    )
    .into()
}
