use bmbp_marco_util::util::*;
use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_tree_bean(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_attrs = struct_input_token.attrs.as_slice();
    let struct_fields = parse_struct_fields(&struct_input_token);
    // 获取树型标记
    let tree_prefix = parse_tree_meta(meta_token.into());
    let mut tree_field_name = build_tree_field_name(tree_prefix);
    let mut tree_field = build_tree_field(tree_field_name.as_slice(), &struct_ident);
    for field in struct_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if !tree_field_name.contains(&field_name) {
            tree_field.push(field);
        }
    }
    let struct_field_token = build_struct_field_token(tree_field.as_slice());
    let struct_method_token = build_struct_props_method_token(tree_field.as_slice());
    build_struct_token(
        struct_ident,
        struct_attrs,
        struct_field_token,
        struct_method_token,
    )
    .into()
}
