use bmbp_marco_util::util::{
    build_struct_field_token, build_struct_props_method_token, build_struct_token,
    build_tree_field, build_tree_field_name, parse_struct_fields, parse_tree_meta,
};
use bmbp_marco_util::{
    build_base_field, build_struct_option_field_token, build_struct_option_props_method_token,
    merge_struct_fields, util,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_rdbc_tree_bean(
    meta_token: TokenStream,
    model_token: TokenStream,
) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_generics = &struct_input_token.generics;
    let struct_attrs = &struct_input_token.attrs.as_slice();
    // 基础字段
    let struct_base_field_name = util::build_base_field_name();
    let mut struct_base_fields = util::build_base_field();

    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token.into());
    let tree_field_name = build_tree_field_name(tree_prefix.clone());
    let tree_field = build_tree_field(tree_field_name.as_slice(), &struct_ident);
    // 合并公共字段
    struct_base_fields.extend_from_slice(tree_field.as_slice());
    let struct_fields = parse_struct_fields(&struct_input_token);
    for field in struct_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if !struct_base_field_name.contains(&field_name) && !tree_field_name.contains(&field_name) {
            struct_base_fields.push(field);
        }
    }
    let struct_field_token = build_struct_field_token(struct_base_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_base_fields.as_slice());
    build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    )
    .into()
}
pub(crate) fn marco_rdbc_tree_bean_option(
    meta_token: TokenStream,
    model_token: TokenStream,
) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_attrs = &struct_input_token.attrs.as_slice();
    let struct_generics = &struct_input_token.generics;
    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token.into());
    let tree_field_name = build_tree_field_name(tree_prefix.clone());
    let tree_field = build_tree_field(tree_field_name.as_slice(), &struct_ident);
    let mut struct_fields = parse_struct_fields(&struct_input_token);
    let struct_base_fields = build_base_field();
    struct_fields = merge_struct_fields(struct_fields, struct_base_fields.as_slice());
    struct_fields = merge_struct_fields(struct_fields, tree_field.as_slice());

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
