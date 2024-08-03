use bmbp_marco_util::util;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn marco_orm_row(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let parse_struct_token = struct_token.clone();
    let struct_input_token = parse_macro_input!(parse_struct_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    let struct_fields = util::parse_struct_fields(&struct_input_token);
    let mut field_set_value_vec: Vec<TokenStream2> = vec![];
    for field in struct_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("set_{}", field_ident);
        let token = quote! {
          if let Some(data) = row.get_data().get(#field_name) {
              model.#field_method(Some(data.into()));
          }
        };
        field_set_value_vec.push(token);
    }
    // 赋值语句集合
    let from_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::new();
                #(#field_set_value_vec)*
                model
            }
        }
    };
    let token_vec = vec![struct_token, from_token.into()];
    let token = TokenStream::from_iter(token_vec);
    println!("=======>{}", token.to_string());
    token
}
