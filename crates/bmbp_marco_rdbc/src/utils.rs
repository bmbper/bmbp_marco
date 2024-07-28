use crate::types::{ValidMeta, ValidRule, ValidRuleMethod};
use bmbp_rdbc::RdbcValidMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Expr, Field, Lit, Meta, Token, Type, TypePath};
use uuid::Uuid;

/// 获取基础模型
pub(crate) fn build_base_struct_token() -> TokenStream {
    let base_model = quote! {
         pub struct BmbpOrmBaseModel {
             #[query(eq)]
             data_id: Option<String>,
             #[query(eq)]
             data_level: Option<String>,
             #[query(eq)]
             data_flag: Option<String>,
             #[query(eq)]
             data_status: Option<String>,
             data_sort: Option<i32>,
             #[query(bet)]
             data_create_time: Option<String>,
             #[query(eq)]
             data_create_user: Option<String>,
             #[query(bet)]
             data_update_time: Option<String>,
             #[query(eq)]
             data_update_user: Option<String>,
             data_owner_org: Option<String>,
             data_sign: Option<String>,
         }
    };
    base_model.into()
}
/// 获取基础树型模型
pub(crate) fn build_base_tree_struct_token() -> TokenStream {
    let base_tree_model = quote! {
         pub struct BmbpOrmTreeModel {
             // 编码
             #[query(eq)]
             code: Option<String>,
             // 上级编码
             #[query(eq)]
             parent_code: Option<String>,
             // 编码路径
             #[query(like)]
             code_path: Option<String>,
             // 名称
             #[query(like)]
             name: Option<String>,
             // 名称路径
             #[query(like)]
             name_path: Option<String>,
             // 层级
             #[query(eq)]
             grade: Option<i32>,
             #[rdbc_skip]
             children: Option<Vec<BmbpOrmTreeModel>>,
         }
    };
    base_tree_model.into()
}
/// 获取结构体字段
pub(crate) fn build_struct_field_cache(field: &[Field]) -> HashMap<String, Field> {
    let mut map = HashMap::new();
    for item in field {
        map.insert(item.ident.clone().unwrap().to_string(), item.clone());
    }
    map
}

/// 判断是否是OptionField
pub(crate) fn field_has_option_type(field_type: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 {
            if path.segments[0].ident.to_string() == "Option" {
                return true;
            }
        }
    }
    false
}

pub(crate) fn field_has_attribute_args(field: &Field, const_attr: &str) -> bool {
    for attr_item in field.attrs.iter() {
        return attr_item.path().is_ident(const_attr);
    }
    false
}
/// 获取结构体字段
pub(crate) fn parse_filed_from_struct(derive_input: &DeriveInput) -> Vec<Field> {
    let mut field_vec = vec![];
    match &derive_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    field_vec.push(field.clone())
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                for field in fields_unnamed.unnamed.iter() {
                    let mut name_field = field.clone();
                    name_field.ident = Some(format_ident!("field_{}", Uuid::new_v4().to_string()));
                    field_vec.push(name_field)
                }
            }
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}

pub(crate) fn parse_query_meta(struct_fields: &Field) -> String {
    let mut field_type = "".to_string();
    for attr_item in struct_fields.attrs.iter() {
        if attr_item.path().is_ident("query") {
            let _ = attr_item.parse_nested_meta(|meta| {
                return if meta.path.is_ident("eq") {
                    field_type = "eq".to_string();
                    Ok(())
                } else if meta.path.is_ident("ne") {
                    field_type = "ne".to_string();
                    Ok(())
                } else if meta.path.is_ident("like") {
                    field_type = "like".to_string();
                    Ok(())
                } else if meta.path.is_ident("like_left") {
                    field_type = "like_left".to_string();
                    Ok(())
                } else if meta.path.is_ident("like_right") {
                    field_type = "like_right".to_string();
                    Ok(())
                } else {
                    Ok(())
                };
            });
        }
    }
    field_type
}

pub(crate) fn parse_field_slice_valid_meta(
    struct_fields: &[Field],
) -> (Vec<ValidMeta>, Vec<ValidMeta>) {
    let mut insert_valid_field = vec![];
    let mut update_valid_field = vec![];
    for field in struct_fields {
        for attr_items in field.attrs.iter() {
            if attr_items.path().is_ident("valid") {
                let (insert_rule, update_rule) = parse_field_item_valid_meta(
                    attr_items.meta.clone(),
                    &ValidRuleMethod::INSERT_UPDATE,
                );
                if !insert_rule.is_empty() {
                    let insert_valid = ValidMeta::new(field.clone(), insert_rule);
                    insert_valid_field.push(insert_valid);
                }
                if !update_rule.is_empty() {
                    let update_valid = ValidMeta::new(field.clone(), update_rule);
                    update_valid_field.push(update_valid);
                }
            }
        }
    }
    (insert_valid_field, update_valid_field)
}
fn parse_field_item_valid_meta(
    attrs: Meta,
    valid_rule_method: &ValidRuleMethod,
) -> (Vec<ValidRule>, Vec<ValidRule>) {
    println!(
        "=====> 字段的校验规则:{}",
        attrs.to_token_stream().to_string()
    );

    // 定义validMeta
    let mut valid_meta = RdbcValidMeta::default();

    let mut insert_rule_vec = vec![];
    let mut update_rule_vec = vec![];
    if let Meta::List(meta_list) = attrs {
        if let Ok(meta_list) =
            meta_list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        {
            for meta_item in meta_list.iter() {
                if meta_item.path().is_ident("name") {
                    match meta_item {
                        Meta::List(child_item_list) => {
                            if let Ok(item_name_list) = child_item_list
                                .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                            {
                                for name_expr in item_name_list.iter() {
                                    if let Expr::Lit(name_expr_lit) = name_expr.clone() {
                                        if let Lit::Str(name_value_str_lit) = name_expr_lit.lit {
                                            valid_meta.name = Some(name_value_str_lit.value());
                                        }
                                    }
                                }
                            }
                            if let Ok(item_name_list) = child_item_list
                                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                            {
                                for name_item_meta in item_name_list.iter() {
                                    match name_item_meta {
                                        Meta::Path(name_item_meta_path) => {
                                            valid_meta.name = Some(
                                                name_item_meta_path
                                                    .get_ident()
                                                    .unwrap()
                                                    .to_string(),
                                            );
                                        }
                                        Meta::List(_) => {
                                            println!("========11>");
                                        }
                                        Meta::NameValue(_) => {
                                            println!("========12>");
                                        }
                                    }
                                }
                            }
                        }
                        Meta::NameValue(name_meta) => {
                            if let Expr::Lit(name_expr_lit) = name_meta.value.clone() {
                                if let Lit::Str(name_value_str_lit) = name_expr_lit.lit {
                                    valid_meta.name = Some(name_value_str_lit.value());
                                }
                            }
                        }

                        Meta::Path(_) => {
                            println!("errror===>");
                        }
                    }
                } else if meta_item.path().is_ident("save") {
                    match meta_item {
                        Meta::List(child_item_list) => {
                            if let Ok(item_name_list) = child_item_list
                                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                            {
                                for name_item_meta in item_name_list.iter() {
                                    let ident =
                                        name_item_meta.path().get_ident().unwrap().to_string();
                                    println!("===save_item:{}", ident);

                                    match name_item_meta {
                                        Meta::List(save_item_list) => {
                                            if let Ok(item_name_list) = save_item_list
                                                .parse_args_with(
                                                    Punctuated::<Meta, Token![,]>::parse_terminated,
                                                )
                                            {
                                                for ii in item_name_list.iter() {
                                                    println!(
                                                        "====save11==>01:{}",
                                                        ii.to_token_stream().to_string()
                                                    );
                                                }
                                            }
                                            if let Ok(item_name_list) = save_item_list
                                                .parse_args_with(
                                                    Punctuated::<Expr, Token![,]>::parse_terminated,
                                                )
                                            {
                                                for ii in item_name_list.iter() {
                                                    println!(
                                                        "====save11=expr=>01:{}",
                                                        ii.to_token_stream().to_string()
                                                    );
                                                }
                                            }
                                        }
                                        Meta::Path(name_item_meta_path) => {
                                            println!("======>save10")
                                        }
                                        Meta::NameValue(_) => {
                                            println!("======>save12")
                                        }
                                    }
                                }
                            }
                        }
                        Meta::Path(_) => {}
                        Meta::NameValue(_) => {}
                    }
                    println!("save===>{}", meta_item.to_token_stream().to_string());
                } else if meta_item.path().is_ident("insert") {
                    println!("insert===>{}", meta_item.to_token_stream().to_string());
                } else if meta_item.path().is_ident("update") {
                    println!("update===>{}", meta_item.to_token_stream().to_string());
                }
            }
        }
    }
    println!("valid==>{:#?}", valid_meta);
    (insert_rule_vec, update_rule_vec)
}

pub(crate) fn filter_field_by_marco_attrs(
    field_slice: &[Field],
    attrs: &str,
    ignore: bool,
) -> Vec<Field> {
    let mut field_vec = vec![];
    for field in field_slice {
        if ignore {
            if !field_has_attribute_args(field, attrs) {
                field_vec.push(field.clone());
            }
        } else {
            if field_has_attribute_args(field, attrs) {
                field_vec.push(field.clone());
            }
        }
    }
    field_vec
}

/// 驼峰转下划线 大写
pub(crate) fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}
