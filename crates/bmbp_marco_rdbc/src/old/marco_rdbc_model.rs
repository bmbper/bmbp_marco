use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::parse::Parser;
use syn::{parse_macro_input, parse_quote, DeriveInput, Field};

use crate::meta::RdbcOrmMeta;
use crate::old::consts::{ATTRS_QUERY, ATTRS_RDBC_SKIP, RDBC_TREE_CHILDREN, RDBC_TREE_CODE, RDBC_TREE_CODE_PATH, RDBC_TREE_NAME, RDBC_TREE_NAME_PATH, RDBC_TREE_NODE_GRADE, RDBC_TREE_PARENT_CODE};
use crate::old::utils::{build_base_struct_token, build_base_tree_struct_token, build_struct_field_cache, camel_to_snake, field_has_attribute_args, field_has_option_type, filter_field_by_marco_attrs, parse_field_slice_valid_meta, parse_filed_from_struct, parse_query_meta};


pub(crate) fn rdbc_model(meta_token: TokenStream, model_token: TokenStream) -> TokenStream {
    // 获取结构体名称
    let struct_input_token = parse_macro_input!(model_token as DeriveInput);
    let struct_ident = &struct_input_token.ident;
    // 获取RDBC_MODEL宏参数
    let mut mode_meta: RdbcOrmMeta = parse_macro_input!(meta_token as RdbcOrmMeta);
    if mode_meta.get_table_name().is_none() || mode_meta.get_table_name().unwrap().is_empty() {
        mode_meta.set_table_name(camel_to_snake(struct_ident.to_string()));
    }
    // 表名称
    let struct_table_name = mode_meta.get_table_name().unwrap();
    // 树前缀
    let struct_tree_prefix = mode_meta.get_tree_prefix();
    // 提取结构体字段
    let mut struct_field_vec = parse_filed_from_struct(&struct_input_token);
    let struct_field_cache = build_struct_field_cache(&struct_field_vec);
    // 树型结构 增加树型字段
    if let Some(ref tree_prefix) = struct_tree_prefix {
        let base_tree_struct_token = build_base_tree_struct_token();
        let base_tree_struct_input = parse_macro_input!(base_tree_struct_token as DeriveInput);
        let base_tree_struct_field_vec = parse_filed_from_struct(&base_tree_struct_input);
        for mut field in base_tree_struct_field_vec {
            let mut field_ident = field.ident.unwrap();
            let mut field_name = field_ident.to_string();
            if field_name.eq(RDBC_TREE_CHILDREN) {
                field.ty = parse_quote! { Option<Vec<#struct_ident>> };
            }
            field_ident = format_ident!("{}_{}", tree_prefix, field_name);
            field.ident = Some(field_ident);
            let field_name = field.ident.as_mut().unwrap().to_string();
            if !struct_field_cache.contains_key(field_name.as_str()) {
                struct_field_vec.push(field);
            }
        }
    }
    // 追加公共字段
    let base_struct_token = build_base_struct_token();
    let base_struct_input = parse_macro_input!(base_struct_token as DeriveInput);
    let base_struct_field_vec = parse_filed_from_struct(&base_struct_input);
    for mut field in base_struct_field_vec {
        let field_name = field.ident.as_mut().unwrap().to_string();
        if !struct_field_cache.contains_key(field_name.as_str()) {
            struct_field_vec.push(field);
        }
    }

    // 构建语法树
    let mut model_struct_macro_token: Vec<TokenStream2> = vec![];

    // 查询结构
    let query_struct = build_query_struct_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(query_struct);
    // 模型结构
    let model_struct_token = build_struct_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(model_struct_token);
    // 构建初始化数据的方法
    let model_data_init_token =
        build_struct_init_data_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(model_data_init_token);

    // 构建结构体-获取表名称、获取字段的方法
    let struct_table_method_token = build_struct_table_method_token(
        struct_ident,
        &struct_table_name,
        filter_field_by_marco_attrs(&struct_field_vec, ATTRS_RDBC_SKIP, true).as_slice(),
    );
    model_struct_macro_token.push(struct_table_method_token);

    if let Some(ref tree_prefix) = struct_tree_prefix {
        let struct_tree_curd_method_token = build_struct_tree_curd_method_token(
            struct_ident,
            struct_field_vec.as_slice(),
            tree_prefix,
        );
        model_struct_macro_token.extend_from_slice(struct_tree_curd_method_token.as_slice());

        let struct_tree_curd_save_method_token = build_struct_tree_curd_save_method_token(
            struct_ident,
            struct_field_vec.as_slice(),
            tree_prefix,
        );
        model_struct_macro_token.push(struct_tree_curd_save_method_token);
    } else {
        let struct_curd_save_method_token =
            build_struct_curd_save_method_token(struct_ident, struct_field_vec.as_slice());
        model_struct_macro_token.push(struct_curd_save_method_token);
    }
    // 增删改查方法
    let struct_curd_method_token =
        build_struct_curd_method_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(struct_curd_method_token);

    // 校验方法
    let struct_valid_method_token =
        build_struct_valid_method_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(struct_valid_method_token);

    // SQL
    let struct_sql_method_token =
        build_struct_sql_method_token(&struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(struct_sql_method_token);
    let struct_orm_token = build_struct_orm_token(&struct_ident);
    model_struct_macro_token.push(struct_orm_token);

    // 构建结构体-RdbcOrmRow的转换
    let struct_impl_for_from_trait_for_rdbc_orm_row_token =
        build_struct_impl_for_from_rdbc_orm_row_token(struct_ident, struct_field_vec.as_slice());
    model_struct_macro_token.push(struct_impl_for_from_trait_for_rdbc_orm_row_token);

    if let Some(ref tree_prefix) = struct_tree_prefix {
        let struct_impl_rdbc_tree_trait_token = build_impl_for_rdbc_tree_trait_token(
            struct_ident,
            struct_tree_prefix.as_ref().unwrap(),
            struct_field_vec.as_slice(),
        );
        model_struct_macro_token.push(struct_impl_rdbc_tree_trait_token);
    }

    /// 构建结构体-handler-web接口方法
    let struct_web_handler_token =
        build_struct_web_handler_token(struct_ident, struct_tree_prefix.is_some());
    model_struct_macro_token.push(struct_web_handler_token);
    /// 构建结构体-router-路由方法
    let model_router_token = build_struct_router_token(struct_ident, struct_tree_prefix.is_some());
    model_struct_macro_token.push(model_router_token);
    let final_token = quote! {
       #(#model_struct_macro_token)*
    };
    println!("最终输出{}", final_token.to_string());
    final_token.into()
}

fn build_query_struct_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    // 构建查询结构体
    let query_struct_ident = format_ident!("{}QueryVo", struct_ident);
    // 查询结构字段
    let query_struct_field_vec = filter_field_by_marco_attrs(struct_fields, ATTRS_QUERY, false);
    build_struct_token(&query_struct_ident, query_struct_field_vec.as_slice())
}
fn build_struct_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_opt_field_vec = build_struct_field_token(struct_fields);
    let struct_method_token = build_struct_field_method_set_get_token(struct_fields);
    quote! {
        #[derive(Default, Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(default)]
        pub struct #struct_ident {
               #(#struct_opt_field_vec,)*
        }

        impl #struct_ident {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
        }
    }
}
fn build_struct_field_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
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

fn build_struct_field_method_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_method_token = build_struct_field_method_set_get_token(struct_fields);
    let token = quote! {
        impl #struct_ident {
            pub fn new() -> Self {
                Self::default()
            }
            #(#struct_method_token)*
        }
    };
    token
}

fn build_struct_field_method_set_get_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
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
                pub fn #get_mut_method_name(&mut self) ->&mut Option<#field_type> {
                    &mut self.#field_name
                }
            }
        };
        method_vec.push(method_token);
    }
    method_vec
}
fn build_struct_init_data_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let init_data_method_token = build_struct_init_data_method_token(struct_fields);
    quote! {
        impl #struct_ident {
            #(#init_data_method_token)*
        }
    }
}
fn build_struct_init_data_method_token(struct_fields: &[Field]) -> Vec<TokenStream2> {
    let mut method_vec = vec![];
    let mut insert_data_value = vec![];
    let mut update_data_value = vec![];
    for item in struct_fields {
        let field_name = item.ident.as_ref().unwrap();
        let field_name_string = field_name.to_string();
        match field_name_string.as_str() {
            RDBC_DATA_ID => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(uuid::Uuid::new_v4().to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_LEVEL => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_FLAG => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }

            RDBC_DATA_STATUS => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("0".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_SORT => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(0);
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_CREATE_TIME => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_CREATE_USER => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token);
            }
            RDBC_DATA_UPDATE_TIME => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                };
                insert_data_value.push(token.clone());
                update_data_value.push(token);
            }
            RDBC_DATA_UPDATE_USER => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
                update_data_value.push(token);
            }
            RDBC_DATA_OWNER_ORG => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
            }
            RDBC_DATA_SIGN => {
                let token = quote! {
                    if self.#field_name.is_none() {
                        self.#field_name = Some("".to_string());
                    }
                };
                insert_data_value.push(token.clone());
            }
            &_ => {}
        }
    }
    method_vec.push(quote! {
        pub fn init_insert_data(&mut self)->&mut Self{
            #(#insert_data_value)*
            self
        }
    });
    method_vec.push(quote! {
        pub fn init_update_data(&mut self)->&mut Self{
            #(#update_data_value)*
            self
        }
    });
    method_vec
}

fn build_struct_sql_method_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let insert_sql_token = build_struct_sql_method_insert_token(struct_fields);
    let update_sql_token = build_struct_sql_method_update_token(struct_fields);
    let sql_token = quote! {
        impl #struct_ident {
            pub fn build_query_sql() -> QueryWrapper {
                let mut query = QueryWrapper::new();
                query.table(Self::get_rdbc_table_name());
                query.select_vec(Self::get_rdbc_table_columns());
                query.order_by("data_sort", true);
                query.order_by("data_update_time", false);
                query
            }
            pub fn build_info_sql(data_id:&Option<String>) -> QueryWrapper {
                let mut query = QueryWrapper::new();
                query.table(Self::get_rdbc_table_name());
                query.select_vec(Self::get_rdbc_table_columns());
                query.eq_(Self::get_table_primary_key(), data_id);
                query
            }
            pub fn build_remove_sql(data_id:&Option<String>) -> DeleteWrapper {
                let mut delete = DeleteWrapper::new();
                delete.table(Self::get_rdbc_table_name());
                delete.eq_(Self::get_table_primary_key(), data_id);
                delete
            }
            pub fn build_enable_sql(data_id:&Option<String>) -> UpdateWrapper {
                let mut update = UpdateWrapper::new();
                update.table(Self::get_rdbc_table_name());
                update.set("data_status", "1");
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_disable_sql(data_id:&Option<String>) -> UpdateWrapper {
                let mut update = UpdateWrapper::new();
                update.table(Self::get_rdbc_table_name());
                update.set("data_status", "0");
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_update_status_sql(data_id:&Option<String>, status: String ) -> UpdateWrapper {
                let mut update = UpdateWrapper::new();
                update.table(Self::get_rdbc_table_name());
                update.set("data_status", status);
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            pub fn build_update_flag_sql(data_id:&Option<String>, flag: String) -> UpdateWrapper {
                let mut update = UpdateWrapper::new();
                update.table(Self::get_rdbc_table_name());
                update.set("data_flag", flag);
                update.eq_(Self::get_table_primary_key(), data_id);
                update
            }
            #insert_sql_token
            #update_sql_token
        }
    };
    sql_token
}
fn build_struct_sql_method_insert_token(struct_fields: &[Field]) -> TokenStream2 {
    let mut insert_field_vec = vec![];
    for field in struct_fields {
        if field_has_attribute_args(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("get_{}", field_ident);
        let insert_item = quote! {
            if let Some(value) = self.#field_method() {
                insert.insert_column_value(#field_name, value);
            }
        };
        insert_field_vec.push(insert_item);
    }
    quote! {
        pub fn build_insert_sql(&self) -> InsertWrapper {
                let mut insert = InsertWrapper::new();
                insert.table(Self::get_rdbc_table_name());
                #(#insert_field_vec)*
                insert
            }
    }
}
fn build_struct_sql_method_update_token(struct_fields: &[Field]) -> TokenStream2 {
    let mut update_field_vec = vec![];
    for field in struct_fields {
        if field_has_attribute_args(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("get_{}", field_ident);
        let insert_item = quote! {
            if let Some(value) = self.#field_method() {
                update.set(#field_name, value);
            }
        };
        update_field_vec.push(insert_item);
    }
    quote! {
        pub fn build_update_sql(&self) -> UpdateWrapper {
                let mut update = UpdateWrapper::new();
                update.table(Self::get_rdbc_table_name());
                #(#update_field_vec)*
                update.eq_(Self::get_table_primary_key(),self.get_data_id());
                update
            }
    }
}

fn build_struct_table_method_token(
    struct_ident: &Ident,
    struct_table_name: &String,
    struct_fields: &[Field],
) -> TokenStream2 {
    println!("========>field:{}", struct_fields.len());
    let mut struct_columns = vec![];
    for field in struct_fields {
        let field_name = field.ident.as_ref().unwrap();
        struct_columns.push(field_name.to_string());
    }
    let token = quote! {
        impl #struct_ident {
            pub fn get_rdbc_table_name() -> String {
                return #struct_table_name.to_string();
            }
            pub fn get_table_primary_key() -> String {
                return "data_id".to_string();
            }
            pub fn get_rdbc_table_columns() -> Vec<String> {
                return vec![
                    #(#struct_columns.to_string(),)*
                ];
            }
        }
    };
    token
}
fn build_struct_curd_method_token(struct_ident: &Ident, struct_fields: &[Field]) -> TokenStream2 {
    let struct_query_filter_sql_token = build_struct_curd_method_filter_token(struct_fields);
    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let token = quote! {
        impl #struct_ident {
            pub async fn find_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                query.eq_("data_flag","0");
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_all_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_removed_page(page_params: BmbpPageParam<#struct_query_ident>) -> BmbpResp<PageVo<Self>> {
                let mut query = #struct_ident::build_query_sql();
                if let Some(query_vo) = page_params.get_params() {
                    #(#struct_query_filter_sql_token)*
                }
                query.eq_("data_flag","-1");
                Self::find_page_by_query(page_params.get_page_no(), page_params.get_page_size(), &query).await
            }
            pub async fn find_page_by_query(page_no: &usize, page_size: &usize,query:&QueryWrapper) -> BmbpResp<PageVo<Self>> {
                #orm_ident::select_page_by_query(page_no, page_size, &query).await
            }

            pub async fn find_list(query_vo: &#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
                query.eq_("data_flag","0");
                Self::find_list_by_query(&query).await
            }
            pub async fn find_all_list(query_vo:&#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
               Self::find_list_by_query(&query).await
            }
            pub async fn find_removed_list(query_vo:&#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                let mut query = #struct_ident::build_query_sql();
                #(#struct_query_filter_sql_token)*
                query.eq_("data_flag","-1");
                Self::find_list_by_query(&query).await
            }
            pub async fn find_list_by_query(query:&QueryWrapper)-> BmbpResp<Option<Vec<Self>>> {
                #orm_ident::select_list_by_query(query).await
            }
            pub async fn find_by_id(id: &Option<String>) -> BmbpResp<Option<Self>> {
                let mut query = #struct_ident::build_query_sql();
                query.eq_(Self::get_table_primary_key(),id);
                #orm_ident::select_one_by_query(&query).await
            }

            pub async fn find_one(&self) -> BmbpResp<Option<Self>> {
                Self::find_by_id(&self.get_data_id()).await
            }
            pub async fn save(&mut self) -> BmbpResp<Option<Self>> {
                let model = self.find_one().await?;
                if model.is_some() {
                    self.update().await?;
                } else {
                    self.insert().await?;
                }
                self.find_one().await
            }
            pub async fn remove(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let remove = Self::build_remove_sql(self.get_data_id());
                #orm_ident::execute_delete(&remove).await
            }
            pub async fn remove_logic(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let update = Self::build_update_flag_sql(self.get_data_id(),"-1".to_string());
                #orm_ident::execute_update(&update).await
            }
            pub async fn enable(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要启用的记录"));
                }
                let update = Self::build_enable_sql(self.get_data_id());
                #orm_ident::execute_update(&update).await
            }
            pub async fn disable(&self) -> BmbpResp<usize> {
                if self.get_data_id().is_none() {
                    return Err(BmbpError::service("请指定要停用的记录"));
                }
                let update = Self::build_disable_sql(self.get_data_id());
                #orm_ident::execute_update(&update).await
            }
            pub async fn remove_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let delete = Self::build_remove_sql(id);
                #orm_ident::execute_delete(&delete).await
            }
            pub async fn remove_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
             pub async fn remove_logic_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要删除的记录"));
                }
                let update = Self::build_update_flag_sql(id,"-1".to_string());
                #orm_ident::execute_update(&update).await
            }
            pub async fn remove_logic_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn enable_by_id(id: &Option<String>) -> BmbpResp<usize> {
                if id.is_none() {
                    return Err(BmbpError::service("请指定要启用的记录"));
                }
                let update = Self::build_enable_sql(id);
                #orm_ident::execute_update(&update).await
            }
            pub async fn enable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
            pub async fn disable_by_id(id: &Option<String>) -> BmbpResp<usize> {
                 if id.is_none() {
                    return Err(BmbpError::service("请指定要停用的记录"));
                }
                let update = Self::build_disable_sql(id);
                #orm_ident::execute_update(&update).await
            }
            pub async fn disable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
                Ok(0)
            }
        }
    };
    token
}
fn build_struct_curd_save_method_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
) -> TokenStream2 {
    let orm_ident = format_ident!("{}Orm", struct_ident);
    quote! {
        impl #struct_ident {
            pub async fn insert(&mut self) -> BmbpResp<usize> {
                println!("insert data:{:?}",self);
                // 初始化数据
                self.init_insert_data();
                let _ = self.insert_valid()?;
                let insert = self.build_insert_sql();
                #orm_ident::execute_insert(&insert).await
            }
            pub async fn update(&mut self) -> BmbpResp<usize> {
                self.init_update_data();
                let _ = self.update_valid()?;
                let update = self.build_update_sql();
                #orm_ident::execute_update(&update).await
            }
        }
    }
}
fn build_struct_tree_curd_method_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
    tree_prefix: &String,
) -> Vec<TokenStream2> {
    let mut tree_token = vec![];
    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let tree_prefix_string = tree_prefix.clone();
    let tree_code = format!("{}_code", tree_prefix_string);
    let tree_method_token = vec![
        quote! {
            pub async fn find_tree(query_vo: &#struct_query_ident) -> BmbpResp<Option<Vec<Self>>> {
                  if let Some(row_list) = Self::find_all_list(query_vo).await? {
                    let tree_list = RdbcMarcoTreeUtil::build_tree::<#struct_ident>(row_list);
                    return Ok(Some(tree_list));
                }
                Ok(None)
            }
        },
        quote! {
            pub async fn find_tree_by_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_by_code(code: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_by_parent_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_by_parent_code(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
             pub async fn find_tree_by_code_path(code_path: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_exclude(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_by_exclude_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_exclude_by_code(code: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_exclude_by_parent_id(data_id: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_tree_exclude_by_parent_code(query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
             pub async fn find_tree_exclude_by_code_path(code_path: &Option<String>,query_vo: &#struct_query_ident) -> BmbpResp<Vec<Self>> {
               Ok(vec![])
            }
        },
        quote! {
            pub async fn find_one_by_id(data_id: &Option<String>) -> BmbpResp<Option<Self>> {
                if let Some(v) = data_id {
                    if v.is_empty(){
                        return Ok(None);
                    }
                    let mut query = #struct_ident::build_query_sql();
                    query.eq_(RDBC_DATA_ID, v);
                    #orm_ident::select_one_by_query(&query).await
                }else{
                    Ok(None)
                }
            }
        },
        quote! {
            pub async fn find_one_by_code(code: &Option<String>) -> BmbpResp<Option<Self>> {
                if let Some(v) = code {
                    if v.is_empty(){
                        return Ok(None);
                    }
                    let mut query = #struct_ident::build_query_sql();
                    query.eq_(#tree_code, code);
                    #orm_ident::select_one_by_query(&query).await
                }else{
                    Ok(None)
                }
            }
        },
    ];
    tree_token.push(quote! {
        impl #struct_ident {
            #(#tree_method_token)*
        }
    });
    tree_token
}
fn build_struct_tree_curd_save_method_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
    tree_prefix: &String,
) -> TokenStream2 {
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let tree_data_for_insert = build_struct_tree_curd_save_insert_token(&orm_ident, tree_prefix);
    let tree_data_for_update = build_struct_tree_curd_save_update_token(&struct_ident, tree_prefix);

    quote! {
        impl #struct_ident {
            #tree_data_for_insert
            #tree_data_for_update
        }

    }
}

fn build_struct_tree_curd_save_insert_token(
    orm_ident: &Ident,
    tree_prefix: &String,
) -> TokenStream2 {
    let mut set_token_vec = vec![];
    let code = format_ident!("{}_{}", tree_prefix, RDBC_TREE_CODE);
    let code_path = format_ident!("{}_{}", tree_prefix, RDBC_TREE_CODE_PATH);
    let parent_code = format_ident!("{}_{}", tree_prefix, RDBC_TREE_PARENT_CODE);
    let name = format_ident!("{}_{}", tree_prefix, RDBC_TREE_NAME);
    let name_path = format_ident!("{}_{}", tree_prefix, RDBC_TREE_NAME_PATH);
    let tree_grade = format_ident!("{}_{}", tree_prefix, RDBC_TREE_NODE_GRADE);
    let code_token = quote! {
        if self.#code.is_none() {
            self.#code = Some(Uuid::new_v4().to_string().to_uppercase().replace("-",""));
        }
    };
    set_token_vec.push(code_token);
    // parent_code
    let parent_token = quote! {
        let mut parent_code_path = "".to_string();
        let mut parent_name_path = "".to_string();
        if let Some(p_code) = self.#parent_code.as_ref(){
            if p_code.is_empty(){
                self.#parent_code = Some(RDBC_TREE_ROOT_NODE.to_string());
                parent_code_path = "/".to_string();
                parent_name_path = "/".to_string();
            }else{
                //TODO 加载上级节点
                let mut node = Self::find_one_by_code(&Some(p_code.to_string())).await?;
                if let Some(v) = node {
                    if let Some(p_code_path) = v.#code_path.clone(){
                        if let Some(code_path_tmp) = v.#code_path.clone() {
                            parent_code_path = format!("{}", code_path_tmp);
                        }
                        if let Some(name_path_tmp) = v.#name_path.clone() {
                            parent_name_path = format!("{}", name_path_tmp);
                        }
                    }
                }else{
                    if p_code != RDBC_TREE_ROOT_NODE{
                        return Err(BmbpError::service("上级节点不存在"));
                    }
                    self.#parent_code = Some(RDBC_TREE_ROOT_NODE.to_string());
                    parent_code_path = "/".to_string();
                    parent_name_path = "/".to_string();
                }
            }
        }else{
            self.#parent_code = Some(RDBC_TREE_ROOT_NODE.to_string());
            parent_code_path = "/".to_string();
            parent_name_path = "/".to_string();
        }
        let code_path = format!("{}{}/", parent_code_path, self.#code.as_ref().unwrap());
        let name_path = format!("{}{}/", parent_name_path, self.#name.as_ref().unwrap());
        self.#code_path = Some(code_path.clone());
        self.#name_path = Some(name_path);
        let tree_grade = code_path.split("/").count() - 2;
        self.#tree_grade = Some(tree_grade as i32);
    };
    set_token_vec.push(parent_token);
    quote! {
        pub async fn insert(&mut self) -> BmbpResp<usize> {
                info!("insert tree data...");
                // 初始化数据
                self.init_insert_data();
                #(#set_token_vec)*
                let _ = self.insert_valid()?;
                let insert = self.build_insert_sql();
                #orm_ident::execute_insert(&insert).await
        }
    }
}
fn build_struct_tree_curd_save_update_token(
    struct_ident: &Ident,
    tree_prefix: &String,
) -> TokenStream2 {
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let parent_code = format_ident!("{}_{}", tree_prefix, RDBC_TREE_PARENT_CODE);
    let code = format_ident!("{}_{}", tree_prefix, RDBC_TREE_CODE);
    let name = format_ident!("{}_{}", tree_prefix, RDBC_TREE_NAME);
    let name_path = format_ident!("{}_{}", tree_prefix, RDBC_TREE_NAME_PATH);
    let code_path = format_ident!("{}_{}", tree_prefix, RDBC_TREE_CODE_PATH);
    let code_path_column = format!("{}_{}", tree_prefix, RDBC_TREE_CODE_PATH);
    let name_path_column = format!("{}_{}", tree_prefix, RDBC_TREE_NAME_PATH);
    quote! {
        pub async fn update(&mut self) -> BmbpResp<usize> {
            let old_node_op = Self::find_one_by_id(&self.data_id).await?;
            if old_node_op.is_none() {
                return Err(BmbpError::service("指定的记录不存在!"));
            }
            let old_node = old_node_op.unwrap();
            // 树型结构不能修改节点编码
            self.#code = None;
            self.init_update_data();
            let _ = self.update_valid()?;

            let mut old_title_path = old_node.#name_path.unwrap().clone();
            let mut old_code_path = old_node.#code_path.unwrap().clone();
            let mut new_title_path = "".to_string();
            let mut new_code_path = "".to_string();

            let mut node_name = old_node.#name.clone().unwrap();
            if let Some(name) = self.#name.clone() {
                node_name = name;
            }

            let mut parent_code = "".to_string();
            // 上级编码
            if let Some(code) = self.#parent_code.clone() {
                parent_code = code;
            }else{
                if let Some(code) = old_node.#parent_code.clone() {
                    parent_code = code;
                }
            }
            if parent_code.is_empty(){
                return Err(BmbpError::service("上级节点不存在"));
            }
            if parent_code.as_str() == RDBC_TREE_ROOT_NODE{
                new_title_path = format!("/{}/", node_name.clone());
                new_code_path = format!("/{}/", old_node.#code.clone().unwrap());
            }else{
                 if let Some(parent_node) = Self::find_one_by_code(&Some(parent_code)).await?{
                    new_title_path = format!("{}{}/",parent_node.#name_path.clone().unwrap(),node_name.clone());
                    new_code_path = format!("{}{}/", parent_node.#code_path.clone().unwrap(),old_node.#code.clone().unwrap());
                }else{
                    return Err(BmbpError::service("指定的上级记录不存在!"));
                }
            }
            let update = self.build_update_sql();
            let mut row_count = #orm_ident::execute_update(&update).await?;
            row_count = row_count + Self::change_node_path(old_title_path,new_title_path,old_code_path,new_code_path).await?;
            Ok(row_count)
        }

        pub async fn update_parent(data_id: &Option<String>,parent_code: &Option<String>) -> BmbpResp<usize> {
            let mut node = #struct_ident::default();
            if let Some(id) = data_id.clone() {
                node.data_id = Some(id);
            }
            if let Some(p_code) = parent_code.clone() {
                node.#parent_code = Some(p_code);
            }
            node.update().await
        }
        pub async fn change_node_path(old_title_path:String,new_title_path:String,old_code_path:String,new_code_path:String) -> BmbpResp<usize> {
            let mut update = UpdateWrapper::new();
            update.table(Self::get_rdbc_table_name())
            .set(#name_path_column,RdbcColumn::replace(#name_path_column,&old_title_path,&new_title_path))
            .set(#code_path_column,RdbcColumn::replace(#code_path_column,&old_code_path,&new_code_path));
            update.like_left_value(#code_path_column, &old_code_path);
            #orm_ident::execute_update(&update).await
        }
    }
}
fn build_struct_curd_method_filter_token(query_fields: &[Field]) -> Vec<TokenStream2> {
    let mut token_vec = vec![];
    for field in query_fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let get_method_name = format_ident!("get_{}", field_ident);
        let query_type: String = parse_query_meta(field);
        match query_type.as_str() {
            "eq" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.eq_(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "ne" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.ne_(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like_left" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like_left_value(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            "like_right" => {
                let query_token = quote! {
                    if let Some(value) = query_vo.#get_method_name() {
                        query.like_right_value(#field_name,value);
                    }
                };
                token_vec.push(query_token);
            }
            _ => {}
        }
    }
    token_vec
}

/// 构建结构体-数据校验方法
fn build_struct_valid_method_token(
    struct_ident: &Ident,
    struct_field_slice: &[Field],
) -> TokenStream2 {
    let (insert_valid, update_valid) = parse_field_slice_valid_meta(struct_field_slice);
    quote! {
        impl #struct_ident {
            pub fn insert_valid(&self) -> BmbpResp<()> {
                Ok(())
            }
            pub fn update_valid(&self) -> BmbpResp<()> {
                Ok(())
            }
        }
    }
}
fn build_struct_impl_for_from_rdbc_orm_row_token(
    struct_ident: &Ident,
    struct_fields: &[Field],
) -> TokenStream2 {
    let mut field_set_value_vec = vec![];
    for field in struct_fields {
        if field_has_attribute_args(field, ATTRS_RDBC_SKIP) {
            continue;
        }
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_method = format_ident!("set_{}", field_ident);
        let t_ = quote! {
          if let Some(data) = row.get_data().get(#field_name) {
              model.#field_method(Some(data.into()));
          }
        };
        field_set_value_vec.push(t_);
    }

    let final_token = quote! {
        impl From<RdbcOrmRow> for #struct_ident {
            fn from(row: RdbcOrmRow) -> Self {
                let mut model = #struct_ident::new();
                #(#field_set_value_vec)*
                model
            }
        }
    };
    final_token
}
fn build_impl_for_rdbc_tree_trait_token(
    struct_ident: &Ident,
    tree_prefix: &String,
    field: &[Field],
) -> TokenStream2 {
    let field_cache = build_struct_field_cache(field);
    let get_field = |field_name: &str| {
        let new_field_name = format!("{}_{}", tree_prefix, field_name.to_string().to_lowercase());
        field_cache.get(&new_field_name)
    };
    let get_field_type = |field: &Field| field.ty.clone();
    let mut method_token_vec = vec![];

    if let Some(field) = get_field(RDBC_TREE_CODE) {
        let field_type = get_field_type(field);
        let field_ident = field.ident.as_ref().unwrap();
        let method_token = quote! {
            fn get_code(&self) -> &#field_type{
                &self.#field_ident
            }
            fn set_code(&mut self, code: #field_type) -> &mut Self{
                self.#field_ident =  code;
                self
            }
        };
        method_token_vec.push(method_token);
    }
    if let Some(field) = get_field(RDBC_TREE_PARENT_CODE) {
        let field_type = get_field_type(field);
        let field_ident = field.ident.as_ref().unwrap();
        let method_token = quote! {
            fn get_parent_code(&self) -> &#field_type{
                &self.#field_ident
            }
            fn set_parent_code(&mut self, code: #field_type) -> &mut Self{
                self.#field_ident =  code;
                self
            }
        };
        method_token_vec.push(method_token);
    }
    if let Some(field) = get_field(RDBC_TREE_CHILDREN) {
        let field_type = get_field_type(field);
        let field_ident = field.ident.as_ref().unwrap();
        let method_token = quote! {
            fn get_children(&self) -> &#field_type{
                &self.#field_ident
            }
            fn get_children_mut(&mut self) -> &mut #field_type{
                &mut self.#field_ident
            }
            fn set_children(&mut self, children: #field_type) -> &mut Self{
                self.#field_ident =  children;
                self
            }
        };
        method_token_vec.push(method_token);
    }
    quote! {
        impl RdbcMacroTree<#struct_ident> for #struct_ident {
            #(#method_token_vec)*
        }
    }
}
fn build_struct_orm_token(struct_ident: &Ident) -> TokenStream2 {
    let orm_ident = format_ident!("{}Orm", struct_ident);
    let token = quote! {
        pub struct #orm_ident;
        impl #orm_ident {
            pub async fn select_page_by_query(
                page_no: &usize,
                page_size: &usize,
                query: &QueryWrapper,
            ) -> BmbpResp<PageVo<#struct_ident>> {
                match RdbcOrmIns
                    .await
                    .select_page_by_query::<#struct_ident>(page_no.clone(), page_size.clone(), query)
                    .await
                {
                    Ok(mut page) => {
                        let mut page_vo = PageVo::new();
                        page_vo.set_page_no(page.page_num().clone());
                        page_vo.set_page_size(page.page_size().clone());
                        page_vo.set_op_data(page.data_take());
                        page_vo.set_row_total(page.total().clone());
                        Ok(page_vo)
                    }
                    Err(e) => Err(BmbpError::service(e.get_msg().as_str())),
                }
            }
                pub async fn select_list_by_query(query: &QueryWrapper) -> BmbpResp<Option<Vec<#struct_ident>>> {
                    match RdbcOrmIns
                        .await
                        .select_list_by_query::<#struct_ident>(query)
                        .await
                    {
                        Ok(data) => Ok(data),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn select_one_by_query(query: &QueryWrapper) -> BmbpResp<Option<#struct_ident>> {
                    match RdbcOrmIns
                        .await
                        .select_one_by_query::<#struct_ident>(query)
                        .await
                    {
                        Ok(data) => Ok(data),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_insert(insert: &InsertWrapper) -> BmbpResp<usize> {
                    match RdbcOrmIns.await.execute_insert(insert).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_update(update: &UpdateWrapper) -> BmbpResp<usize> {
                    match RdbcOrmIns.await.execute_update(update).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
                pub async fn execute_delete(delete: &DeleteWrapper) -> BmbpResp<usize> {
                    match RdbcOrmIns.await.execute_delete(delete).await {
                        Ok(data) => Ok(data as usize),
                        Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
                    }
                }
        }
    };
    token
}
fn build_struct_web_handler_token(struct_ident: &Ident, is_tree: bool) -> TokenStream2 {
    let struct_name = camel_to_snake(struct_ident.to_string()).to_lowercase();
    let find_all_page_name = format_ident!("{}_find_all_page", struct_name);
    let find_removed_page_name = format_ident!("{}_find_removed_page", struct_name);
    let find_page_name = format_ident!("{}_find_page", struct_name);
    let find_all_list_name = format_ident!("{}_find_all_list", struct_name);
    let find_removed_list_name = format_ident!("{}_find_removed_list", struct_name);
    let find_list_name = format_ident!("{}_find_list", struct_name);
    let find_info_name = format_ident!("{}_find_info", struct_name);
    let save_name = format_ident!("{}_save", struct_name);
    let save_batch_name = format_ident!("{}_save_batch", struct_name);
    let insert_name = format_ident!("{}_insert", struct_name);
    let insert_batch_name = format_ident!("{}_insert_batch", struct_name);
    let update_name = format_ident!("{}_update", struct_name);
    let update_batch_name = format_ident!("{}_update_batch", struct_name);
    let remove_name = format_ident!("{}_remove", struct_name);
    let remove_batch_name = format_ident!("{}_remove_batch", struct_name);
    let remove_logic_name = format_ident!("{}_remove_logic", struct_name);
    let remove_logic_batch_name = format_ident!("{}_remove_logic_batch", struct_name);
    let enable_name = format_ident!("{}_enable", struct_name);
    let disable_name = format_ident!("{}_disable", struct_name);

    let struct_query_ident = format_ident!("{}QueryVo", struct_ident);

    let mut tree_handler = vec![];
    if (is_tree) {
        let find_tree_name = format_ident!("{}_find_tree", struct_name);
        let find_tree_by_id_name = format_ident!("{}_find_tree_by_id", struct_name);
        let find_tree_by_code = format_ident!("{}_find_tree_by_code", struct_name);
        let find_tree_by_parent_code = format_ident!("{}_find_tree_by_parent_code", struct_name);
        let find_tree_by_parent_id = format_ident!("{}_find_tree_by_parent_id", struct_name);
        let find_tree_by_code_path = format_ident!("{}_find_tree_by_code_path", struct_name);

        let find_tree_exclude_name = format_ident!("{}_find_tree_exclude", struct_name);
        let find_tree_exclude_by_id_name = format_ident!("{}_find_tree_exclude_by_id", struct_name);
        let find_tree_exclude_by_code = format_ident!("{}_find_tree_exclude_by_code", struct_name);
        let find_tree_exclude_by_parent_code =
            format_ident!("{}_find_tree_exclude_by_parent_code", struct_name);
        let find_tree_exclude_by_parent_id =
            format_ident!("{}_find_exclude_tree_by_parent_id", struct_name);
        let find_tree_exclude_by_code_path =
            format_ident!("{}_find_exclude_tree_by_code_path", struct_name);

        let find_tree_node_by_id = format_ident!("{}_find_one_by_id", struct_name);
        let find_tree_node_by_code = format_ident!("{}_find_one_by_code", struct_name);

        tree_handler = vec![
            quote! {
                #[handler]
                pub async fn #find_tree_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    let mut query_params = req.parse_json::<#struct_query_ident>().await?;
                    let model_vo = #struct_ident::find_tree(&query_params).await?;
                    Ok(RespVo::ok_find_option(model_vo))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_id_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_parent_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_parent_id(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_by_code_path(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_id_name(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_parent_code(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_parent_id(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_exclude_by_code_path(req: &mut Request, resp: &mut Response) -> HttpRespListVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                #[handler]
                pub async fn #find_tree_node_by_id(req: &mut Request, resp: &mut Response) -> HttpRespVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
            quote! {
                 #[handler]
                pub async fn #find_tree_node_by_code(req: &mut Request, resp: &mut Response) -> HttpRespVo<#struct_ident> {
                    Ok(RespVo::ok_find_option(None))
                }
            },
        ]
    }
    quote! {
        #(#tree_handler)*

        #[handler]
        pub async fn #find_all_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            info!("find_all_page");
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_all_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_removed_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_removed_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_page_name(req: &mut Request, resp: &mut Response) -> HttpRespPageVo<#struct_ident> {
            let mut page_query_params = req.parse_json::<BmbpPageParam<#struct_query_ident>>().await?;
            let page_vo = #struct_ident::find_page(page_query_params).await?;
            Ok(RespVo::ok_find_data(page_vo))
        }

        #[handler]
        pub async fn #find_all_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_all_list(&query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_removed_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_removed_list(&query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_list_name(req: &mut Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
            let mut query_params = req.parse_json::<#struct_query_ident>().await?;
            let model_vo = #struct_ident::find_list(&query_params).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }

        #[handler]
        pub async fn #find_info_name(req: &mut Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            let data_id = req.param::<String>("dataId");
            let model_vo = #struct_ident::find_by_id(&data_id).await?;
            Ok(RespVo::ok_find_option(model_vo))
        }
        #[handler]
        pub async fn #save_name(req: &mut Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            let mut save_vo = req.parse_json::<#struct_ident>().await?;
            Ok(RespVo::ok_save_option(save_vo.save().await?))
        }
        #[handler]
        pub async fn #save_batch_name(req: &mut Request, resp: &mut Response, ) ->HttpRespListVo<#struct_ident>  {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #insert_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
            Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #insert_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #update_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #update_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespListVo<#struct_ident> {
             Ok(RespVo::ok_save_option(None))
        }
        #[handler]
        pub async fn #remove_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::remove_by_id(&data_id).await?;
             Ok(RespVo::ok_remove_data(row_count))
        }
        #[handler]
        pub async fn #remove_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             Ok(RespVo::ok_remove_option(None))
        }
        #[handler]
        pub async fn #remove_logic_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::remove_by_id(&data_id).await?;
             Ok(RespVo::ok_remove_data(row_count))
        }
        #[handler]
        pub async fn #remove_logic_batch_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
              Ok(RespVo::ok_remove_option(None))
        }
        #[handler]
        pub async fn #enable_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
             let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::enable_by_id(&data_id).await?;
              Ok(RespVo::ok_enable_data(row_count))
        }
        #[handler]
        pub async fn #disable_name(req: &Request, resp: &mut Response, ) -> HttpRespVo<usize> {
              let data_id = req.param::<String>("dataId");
             let row_count = #struct_ident::disable_by_id(&data_id).await?;
              Ok(RespVo::ok_disable_data(row_count))
        }
    }
}

fn build_struct_router_token(struct_ident: &Ident, is_tree: bool) -> TokenStream2 {
    let struct_name = camel_to_snake(struct_ident.to_string()).to_lowercase();
    let find_all_page_name = format_ident!("{}_find_all_page", struct_name);
    let find_removed_page_name = format_ident!("{}_find_removed_page", struct_name);
    let find_page_name = format_ident!("{}_find_page", struct_name);
    let find_all_list_name = format_ident!("{}_find_all_list", struct_name);
    let find_removed_list_name = format_ident!("{}_find_removed_list", struct_name);
    let find_list_name = format_ident!("{}_find_list", struct_name);
    let find_info_name = format_ident!("{}_find_info", struct_name);
    let save_name = format_ident!("{}_save", struct_name);
    let save_batch_name = format_ident!("{}_save_batch", struct_name);
    let insert_name = format_ident!("{}_insert", struct_name);
    let insert_batch_name = format_ident!("{}_insert_batch", struct_name);
    let update_name = format_ident!("{}_update", struct_name);
    let update_batch_name = format_ident!("{}_update_batch", struct_name);
    let remove_name = format_ident!("{}_remove", struct_name);
    let remove_batch_name = format_ident!("{}_remove_batch", struct_name);
    let remove_logic_name = format_ident!("{}_remove_logic", struct_name);
    let remove_logic_batch_name = format_ident!("{}_remove_logic_batch", struct_name);
    let enable_name = format_ident!("{}_enable", struct_name);
    let disable_name = format_ident!("{}_disable", struct_name);
    let mut tree_router = vec![];
    if (is_tree) {
        let find_tree_name = format_ident!("{}_find_tree", struct_name);
        let find_tree_by_id_name = format_ident!("{}_find_tree_by_id", struct_name);
        let find_tree_by_code = format_ident!("{}_find_tree_by_code", struct_name);
        let find_tree_by_parent_code = format_ident!("{}_find_tree_by_parent_code", struct_name);
        let find_tree_by_parent_id = format_ident!("{}_find_tree_by_parent_id", struct_name);
        let find_tree_by_code_path = format_ident!("{}_find_tree_by_code_path", struct_name);

        let find_tree_exclude_name = format_ident!("{}_find_tree_exclude", struct_name);
        let find_tree_exclude_by_id_name = format_ident!("{}_find_tree_exclude_by_id", struct_name);
        let find_tree_exclude_by_code = format_ident!("{}_find_tree_exclude_by_code", struct_name);
        let find_tree_exclude_by_parent_code =
            format_ident!("{}_find_tree_exclude_by_parent_code", struct_name);
        let find_tree_exclude_by_parent_id =
            format_ident!("{}_find_exclude_tree_by_parent_id", struct_name);
        let find_tree_exclude_by_code_path =
            format_ident!("{}_find_exclude_tree_by_code_path", struct_name);

        tree_router.push(quote! {
             .push(Router::with_path("find/tree").post(#find_tree_name))
             .push(Router::with_path("find/tree/id/<dataId>").post(#find_tree_by_id_name))
             .push(Router::with_path("find/tree/code/<code>").post(#find_tree_by_code))
             .push(Router::with_path("find/tree/parent/id/<dataId>").post(#find_tree_by_parent_id))
             .push(Router::with_path("find/tree/parent/code/<code>").post(#find_tree_by_parent_code))
             .push(Router::with_path("find/tree/path/<codePath>").post(#find_tree_by_code_path))
             .push(Router::with_path("find/tree/exclude").post(#find_tree_exclude_name))
             .push(Router::with_path("find/tree/exclude/id/<dataId>").post(#find_tree_exclude_by_id_name))
             .push(Router::with_path("find/tree/exclude/code/<code>").post(#find_tree_exclude_by_code))
             .push(Router::with_path("find/tree/exclude/parent/id/<dataId>").post(#find_tree_exclude_by_parent_id))
             .push(Router::with_path("find/tree/exclude/parent/code/<code>").post(#find_tree_exclude_by_parent_code))
             .push(Router::with_path("find/tree/exclude/path/<codePath>").post(#find_tree_exclude_by_code_path))
        });
    }

    quote! {
        impl #struct_ident {
            pub fn build_router() -> Router {
               Router::new()
                .push(Router::with_path("find/all/page").post(#find_all_page_name))
                .push(Router::with_path("find/removed/page").post(#find_removed_page_name))
                .push(Router::with_path("find/page").post(#find_page_name))
                .push(Router::with_path("find/all/list").post(#find_all_list_name))
                .push(Router::with_path("find/removed/list").post(#find_removed_list_name))
                .push(Router::with_path("find/list").post(#find_list_name))
                .push(Router::with_path("find/info/id/<dataId>").post(#find_info_name))
                .push(Router::with_path("save").post(#save_name))
                .push(Router::with_path("save/batch").post(#save_batch_name))
                .push(Router::with_path("insert").post(#insert_name))
                .push(Router::with_path("insert/batch").post(#insert_batch_name))
                .push(Router::with_path("update").post(#update_name))
                .push(Router::with_path("update/batch").post(#update_batch_name))
                .push(Router::with_path("remove/id/<dataId>").post(#remove_name))
                .push(Router::with_path("remove/batch/id").post(#remove_batch_name))
                 .push(Router::with_path("remove/logic/id/<dataId>").post(#remove_logic_name))
                .push(Router::with_path("remove/logic/batch/id").post(#remove_logic_batch_name))
                .push(Router::with_path("enable/id/<dataId>").post(#enable_name))
                .push(Router::with_path("disable/id/<dataId>").post(#disable_name))
                #(#tree_router)*
            }
        }
    }
}
