use crate::utils::camel_to_snake;
use syn::parse::Parse;
use syn::{Expr, Lit, Meta, MetaNameValue, Token};

#[derive(Debug, Default, Clone)]
pub struct RdbcOrmMeta {
    table_name: Option<String>,
    tree_prefix: Option<String>,
}

impl RdbcOrmMeta {
    pub fn new(table_name: String, tree_prefix: String) -> Self {
        RdbcOrmMeta {
            table_name: Some(table_name),
            tree_prefix: Some(tree_prefix),
        }
    }
    pub fn get_table_name(&self) -> Option<String> {
        self.table_name.clone()
    }
    pub fn get_tree_prefix(&self) -> Option<String> {
        self.tree_prefix.clone()
    }
    pub fn set_table_name(&mut self, table_name: String) {
        self.table_name = Some(table_name);
    }
    pub fn set_tree_prefix(&mut self, tree_prefix: String) {
        self.tree_prefix = Some(tree_prefix);
    }
}
impl Parse for RdbcOrmMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // NameValue的赋值函数
        let set_token_value = |token: &mut Option<String>, value: MetaNameValue| {
            if let Expr::Lit(lit) = value.value {
                if let Lit::Str(lit_str) = lit.lit {
                    *token = Some(lit_str.value());
                }
            } else {
                if let Expr::Path(path) = value.value {
                    if let Some(ident) = path.path.get_ident() {
                        *token = Some(ident.to_string());
                    }
                }
            }
        };

        // 表名称
        let mut table_name = None;
        // 树名称
        let mut tree_prefix = None;
        // 其它名称
        let mut others = Vec::new();

        if input.is_empty() {
            return Ok(RdbcOrmMeta {
                table_name,
                tree_prefix,
            });
        }
        while !input.is_empty() {
            if let Ok(meta) = input.parse::<Meta>() {
                match meta {
                    Meta::NameValue(name_value) => {
                        if name_value.path.is_ident("table") {
                            set_token_value(&mut table_name, name_value)
                        } else if name_value.path.is_ident("tree") {
                            set_token_value(&mut tree_prefix, name_value)
                        }
                    }
                    Meta::Path(path) => {
                        if let Some(ident) = path.get_ident() {
                            others.push(ident.to_string());
                        }
                    }
                    Meta::List(_) => {}
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        for item in others {
            if table_name.is_none() {
                table_name = Some(item);
            } else if tree_prefix.is_none() {
                tree_prefix = Some(item);
            }
        }
        if let Some(table) = table_name {
            let snake_table = camel_to_snake(table.to_lowercase());
            table_name = Some(snake_table);
        }
        if let Some(tree) = tree_prefix {
            tree_prefix = Some(tree.to_lowercase());
        }
        Ok(RdbcOrmMeta {
            table_name,
            tree_prefix,
        })
    }
}
