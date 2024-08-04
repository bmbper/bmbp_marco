use proc_macro::TokenStream;
use case_style::CaseStyle;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{Field};
use syn::__private::TokenStream2;
use bmbp_marco_util::{field_has_attrs_ident};
use crate::meta::RdbcTableTreeMeta;

pub(crate) fn build_table_name(table_tree_meta: &RdbcTableTreeMeta, struct_ident: &Ident) -> String {
    match table_tree_meta.get_table() {
        Some(table) => {
            if table.is_empty() {
                CaseStyle::guess(struct_ident.to_string())
                    .unwrap()
                    .to_snakecase()
                    .to_uppercase()
            } else {
                table.to_string()
            }
        }
        None => CaseStyle::guess(struct_ident.to_string())
            .unwrap()
            .to_snakecase()
            .to_uppercase(),
    }
}

pub(crate) fn build_struct_table_token(struct_ident: &Ident, table_name: &String, struct_fields: &[Field]) -> TokenStream {
    let struct_column = build_struct_column_enum(struct_ident,struct_fields);
    let impl_rdbc_ident = build_impl_rdbc_ident(struct_ident, struct_fields);
    let impl_rdbc_table = build_impl_rdbc_table(struct_ident, &table_name, struct_fields);
    let token = quote! {
        #struct_column
        #impl_rdbc_ident
        #impl_rdbc_table
    };
    token.into()
}

pub(crate) fn build_struct_column_enum(struct_ident: &Ident, fields: &[Field]) -> TokenStream2 {
    let struct_columns_name = format_ident!("{}Column", struct_ident);
    let column_fields = build_struct_column_enum_field_ident(fields);
    let token = quote! {
        pub enum #struct_columns_name {
            #(#column_fields),*
        }
    };
    token
}

pub(crate) fn build_impl_rdbc_ident(struct_ident: &Ident, fields: &[Field]) -> TokenStream2 {
    let struct_columns_ident = format_ident!("{}Column", struct_ident);
    let match_column_fields = build_impl_rdbc_ident_field_ident(fields);
    let token = quote! {
        impl RdbcIdent for #struct_columns_ident {
            fn get_ident(&self) -> String {
                match self {
                    #(#match_column_fields),*
                }
            }
        }
    };
    token
}

pub(crate) fn build_impl_rdbc_table(struct_ident: &Ident, table_name: &String, fields: &[Field]) -> TokenStream2 {
    let struct_columns_ident = format_ident!("{}Column", struct_ident);
    let mut primary_key = build_primary_key(fields);
    let mut key_method = vec![];
    if !primary_key.is_empty() {
        if primary_key.len() == 1 {
            let id = primary_key[0].clone();
            let token = quote! {
                fn get_primary_key() -> impl RdbcIdent {
                    #id.to_string()
                }
                 fn get_union_key() -> Vec<impl RdbcIdent> {
                    vec![#id.to_string()]
                }
            };
            key_method.push(token);
        } else {
            let token = quote! {
                fn get_union_key() -> Vec<impl RdbcIdent> {
                    vec![
                        #(#primary_key.to_string()),*
                    ]
                }
            };
            key_method.push(token);
        }
    };

    let mut match_column_fields = build_impl_rdbc_table_field_ident(fields);
    let token = quote! {
        impl RdbcTable for #struct_ident {
            fn get_table() -> impl RdbcIdent {
                #table_name.to_string()
            }
            fn get_columns() -> Vec<impl RdbcIdent> {
                vec![
                    #(#struct_columns_ident::#match_column_fields),*
                ]
            }
            #(#key_method)*
        }
    };
    token
}

fn build_primary_key(fields: &[Field]) -> Vec<String> {
    let mut primary_key = vec![];
    let mut has_data_id = false;
    for field in fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        if field_has_attrs_ident(field, "id") || field_has_attrs_ident(field, "primary_key") {
            primary_key.push(field_name.clone());
        }
        if field_name.as_str() == "data_id" {
            has_data_id = true;
        }
    }
    if primary_key.is_empty() {
        if has_data_id {
            primary_key.push("data_id".to_string());
        }
    }
    primary_key
}

fn parse_struct_table_name(meta: &TokenStream, struct_ident: &Ident) -> String {
    let mut table_name = meta.to_string().replace("\"", "");
    if table_name.is_empty() {
        table_name = struct_ident.to_string();
    }
    table_name = CaseStyle::guess(table_name).unwrap().to_snakecase().to_uppercase();
    table_name
}

fn build_struct_column_enum_field_ident(fields: &[Field]) -> Vec<Ident> {
    let mut column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name).unwrap().to_pascalcase();
        column_fields.push(format_ident!("{}",enum_vars))
    }
    column_fields
}

fn build_impl_rdbc_ident_field_ident(fields: &[Field]) -> Vec<TokenStream2> {
    let mut column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name.clone()).unwrap().to_pascalcase();
        let enum_ident = format_ident!("{}",enum_vars);
        let token = quote! {
            Self::#enum_ident => #field_name.to_string()
        };
        column_fields.push(token)
    }
    column_fields
}

fn build_impl_rdbc_table_field_ident(fields: &[Field]) -> Vec<Ident> {
    let mut match_column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name).unwrap().to_pascalcase();
        let ident = format_ident!("{}",enum_vars);
        match_column_fields.push(ident)
    }
    match_column_fields
}
