use bmbp_marco_util::{build_base_field, build_struct_option_field_token, build_struct_option_props_method_token, merge_struct_fields, util};
use bmbp_marco_util::util::{
    build_struct_field_token, build_struct_props_method_token, parse_struct_fields,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_rdbc_bean(_: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_attrs = &struct_input_token.attrs.as_slice();
    let mut struct_fields = parse_struct_fields(&struct_input_token);
    let mut struct_base_fields = build_base_field();
    struct_fields = merge_struct_fields(struct_fields,struct_base_fields.as_slice());

    let struct_field_token = build_struct_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_fields.as_slice());
    util::build_struct_token(
        struct_ident,
        struct_attrs,
        struct_field_token,
        struct_method_token,
    )
    .into()
}
pub(crate) fn marco_rdbc_bean_option(_: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_attrs = &struct_input_token.attrs.as_slice();

    // 基础字段
    let mut struct_fields = parse_struct_fields(&struct_input_token);
    let struct_base_fields = build_base_field();
    struct_fields = merge_struct_fields(struct_fields,struct_base_fields.as_slice());

    let struct_field_token = build_struct_option_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_option_props_method_token(struct_fields.as_slice());
    util::build_struct_token(
        struct_ident,
        struct_attrs,
        struct_field_token,
        struct_method_token,
    )
        .into()
}
