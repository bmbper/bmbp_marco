use bmbp_marco_util::util::{
    build_struct_field_token, build_struct_props_method_token, parse_struct_fields,
};
use bmbp_marco_util::{
    build_struct_option_field_token, build_struct_option_props_method_token, util,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_bean(_: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_generics = &struct_input_token.generics;
    let struct_attrs = &struct_input_token.attrs.as_slice();
    let struct_fields = parse_struct_fields(&struct_input_token);
    let struct_field_token = build_struct_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_props_method_token(struct_fields.as_slice());
    let token: TokenStream = util::build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    )
    .into();
    println!("AAAAAAAAA=====>{}", token.to_string());
    token
}

pub(crate) fn marco_option_bean(_: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_generics = &struct_input_token.generics;
    let struct_attrs = &struct_input_token.attrs.as_slice();
    let struct_fields = parse_struct_fields(&struct_input_token);
    let struct_field_token = build_struct_option_field_token(struct_fields.as_slice());
    let struct_method_token = build_struct_option_props_method_token(struct_fields.as_slice());
    let token = util::build_struct_token(
        struct_ident,
        struct_attrs,
        struct_generics,
        struct_field_token,
        struct_method_token,
    )
    .into();
    token
}
