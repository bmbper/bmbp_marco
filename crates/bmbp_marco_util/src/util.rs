use std::collections::HashSet;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_quote, Attribute, DeriveInput, Field, FieldMutability, Type, TypePath, Generics};
use crate::util;

/// parse_tree_meta 获取树型标记
pub fn parse_tree_meta(meta_token: TokenStream) -> String {
    meta_token.to_string().replace("\"", "")
}

pub fn parse_struct_fields(struct_input: &DeriveInput) -> Vec<Field> {
    let mut field_vec = vec![];
    match &struct_input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    field_vec.push(field.clone())
                }
            }
            syn::Fields::Unnamed(_) => {}
            syn::Fields::Unit => {}
        },
        _ => {}
    }
    field_vec
}

pub fn build_tree_field_name(tree_prefix: String) -> Vec<String> {
    vec![
        format!("{}_code", tree_prefix),
        format!("{}_parent_code", tree_prefix),
        format!("{}_name", tree_prefix),
        format!("{}_code_path", tree_prefix),
        format!("{}_name_path", tree_prefix),
        format!("{}_tree_grade", tree_prefix),
        format!("{}_leaf", tree_prefix),
        format!("{}_type", tree_prefix),
        format!("{}_children", tree_prefix),
    ]
}

pub fn build_struct_field_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut field_vec = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        field_vec.push(quote! {
             #field_ident: #field_type
        });
    }
    field_vec
}

pub fn build_struct_option_field_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut field_vec = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if field_has_option_type(field_type) {
            field_vec.push(quote! {
                 #field_ident: #field_type
            });
        } else {
            field_vec.push(quote! {
                 #field_ident: Option<#field_type>
            });
        }
    }
    field_vec
}

pub fn build_struct_props_method_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let set_method_name = format_ident!("set_{}", field_name);
        let get_method_name = format_ident!("get_{}", field_name);
        let get_mut_method_name = format_ident!("get_mut_{}", field_name);
        let field_type = &item.ty;
        let method_token = quote! {
            pub fn #set_method_name(&mut self, value: #field_type) -> &mut Self {
                self.#field_name = value;
                self
            }
            pub fn #get_method_name(&self) -> &#field_type {
                &self.#field_name
            }
            pub fn #get_mut_method_name(&mut self) -> &mut #field_type {
                &mut self.#field_name
            }
        };
        method_vec.push(method_token);
    }
    method_vec
}

pub fn build_struct_option_props_method_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let set_method_name = format_ident!("set_{}", field_name);
        let get_method_name = format_ident!("get_{}", field_name);
        let get_mut_method_name = format_ident!("get_mut_{}", field_name);
        let field_type = &item.ty;
        let method_token = if field_has_option_type(field_type) {
            quote! {
                pub fn #set_method_name(&mut self, value: #field_type) -> &mut Self {
                    self.#field_name = value;
                    self
                }
                pub fn #get_method_name(&self) -> &#field_type {
                    &self.#field_name
                }
                pub fn #get_mut_method_name(&mut self) -> &mut #field_type {
                    &mut self.#field_name
                }
            }
        } else {
            quote! {
                pub fn #set_method_name(&mut self, value: Option<#field_type>) -> &mut Self {
                    self.#field_name = value;
                    self
                }
                pub fn #get_method_name(&self) -> &Option<#field_type> {
                    &self.#field_name
                }
                pub fn #get_mut_method_name(&mut self) -> &mut Option<#field_type> {
                    &mut self.#field_name
                }
            }
        };
        method_vec.push(method_token);
    }
    method_vec
}

pub fn build_struct_token(
    struct_ident: &Ident,
    struct_attrs: &[Attribute],
    struct_generics: &Generics,
    struct_field_token: Vec<TokenStream2>,
    struct_method_token: Vec<TokenStream2>,
) -> TokenStream {
    let generics_param_token = util::build_generics_param_token(struct_generics);
    let generics_where_token = util::build_generics_where_token(struct_generics);
    let attrs_token = if struct_attrs.is_empty() {
        quote! {
            #[derive(Default, Debug, Clone, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[serde(default)]
        }
    } else {
        quote! {
            #(#struct_attrs)*
        }
    };
    let bean_token = quote! {
        #attrs_token
        pub struct #struct_ident  #generics_param_token #generics_where_token {
               #(#struct_field_token,)*
        }
        impl #generics_param_token #struct_ident #generics_param_token #generics_where_token {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
        }
    };
    bean_token.into()
}

fn build_generics_param_token(struct_generics: &Generics) -> TokenStream {
    let params_empty = struct_generics.params.is_empty();
    if params_empty {
        return quote! {};
    }
    let params = &struct_generics.params;
    quote! {
        <#params>
    }
}

fn build_generics_where_token(struct_generics: &Generics) -> TokenStream {
    if struct_generics.where_clause.is_none() {
        return quote! {};
    }
    let where_clause = &struct_generics.where_clause;
    quote! {
        #where_clause
    }
}

pub fn build_base_field_name() -> Vec<String> {
    vec![
        "data_id".to_string(),
        "data_level".to_string(),
        "data_flag".to_string(),
        "data_status".to_string(),
        "data_sort".to_string(),
        "data_create_time".to_string(),
        "data_create_user".to_string(),
        "data_update_time".to_string(),
        "data_update_user".to_string(),
        "data_owner_org".to_string(),
        "data_sign".to_string(),
    ]
}

pub fn build_base_field() -> Vec<Field> {
    let field_names = build_base_field_name();
    let mut field_vec = vec![];
    for item in field_names {
        let field_ident = format_ident!("{}", item);
        let field_type = if item.eq("data_sort") {
            parse_quote!(Option<i32>)
        } else {
            parse_quote!(Option<String>)
        };
        let field = Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(field_ident),
            colon_token: None,
            ty: field_type,
        };
        field_vec.push(field)
    }
    field_vec
}

pub fn build_tree_field(filed_names: &[String], struct_name: &Ident) -> Vec<Field> {
    build_tree_field_with_children_skip(filed_names, struct_name, false)
}
pub fn build_tree_field_for_orm(filed_names: &[String], struct_name: &Ident) -> Vec<Field> {
    build_tree_field_with_children_skip(filed_names, struct_name, true)
}

pub fn build_tree_field_with_children_skip(filed_names: &[String], struct_name: &Ident, skip_children: bool) -> Vec<Field> {
    let mut field_vec = vec![];

    for item in filed_names {
        let mut attrs = vec![];
        let field_ident = format_ident!("{}", item);
        let field_type = if item.ends_with("_children") {
            if skip_children {
                attrs.push(parse_quote!(#[skip]));
            }
            parse_quote!(Option<Vec<#struct_name>>)
        } else if item.ends_with("_grade") {
            parse_quote!(Option<u32>)
        } else {
            parse_quote!(Option<String>)
        };
        let field = Field {
            attrs: attrs.clone(),
            vis: syn::Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(field_ident),
            colon_token: None,
            ty: field_type,
        };
        field_vec.push(field)
    }
    field_vec
}

pub fn field_has_option_type(field_type: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 {
            if path.segments[0].ident.to_string() == "Option" {
                return true;
            }
        }
    }
    false
}

pub fn field_has_attrs_ident(field: &Field, attrs: &str) -> bool {
    for attr_item in field.attrs.iter() {
        return attr_item.path().is_ident(attrs);
    }
    false
}

pub fn build_field_name_set(fields: &[Field]) -> HashSet<String> {
    let mut field_name_set = HashSet::new();
    for item in fields {
        if let Some(field_ident) = &item.ident {
            field_name_set.insert(field_ident.to_string());
        }
    }
    field_name_set
}

pub fn merge_struct_fields(mut struct_fields: Vec<Field>, from_fields: &[Field]) -> Vec<Field> {
    let field_name_set = build_field_name_set(struct_fields.as_slice());
    for field in from_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if !field_name_set.contains(&field_name) {
            struct_fields.push(field.clone());
        }
    }
    struct_fields
}