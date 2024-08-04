use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Field};
use syn::__private::TokenStream2;
use bmbp_marco_util::field_has_option_type;

pub fn build_impl_orm_row_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream {
    let mut field_set_value_vec: Vec<TokenStream2> = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("set_{}", field_ident);
        let token = if field_has_option_type(&field.ty) {
            quote! {
                  if let Some(data) = row.get_data().get(#field_name) {
                      model.#field_method(Some(data.into()));
                  }
            }
        } else {
            quote! {
                  if let Some(data) = row.get_data().get(#field_name) {
                      model.#field_method(data.into());
                  }
            }
        };
        field_set_value_vec.push(token);
    }
    // 赋值语句集合
    let orm_row_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::new();
                #(#field_set_value_vec)*
                model
            }
        }
    };
    orm_row_token
}