
use serde::Serialize;
use std::fmt::Debug;
use bmbp_rdbc_orm::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcError, RdbcOrmRow, RdbcPage, UpdateWrapper};

pub trait RdbcActiveModel<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    async fn get_table_name() -> String;
    async fn get_table_columns() -> Vec<String>;
    async fn query_wrapper(&self) -> QueryWrapper;
    async fn insert_wrapper(&self) -> InsertWrapper;
    async fn insert_sensitive_wrapper(&self) -> InsertWrapper;
    async fn update_wrapper(&self) -> UpdateWrapper;
    async fn update_sensitive_wrapper(&self) -> UpdateWrapper;
    async fn delete_wrapper(&self) -> DeleteWrapper;
    async fn save(&mut self) -> Result<usize, RdbcError>;
    async fn save_sensitive(&mut self) -> Result<usize, RdbcError>;
    async fn insert(&mut self) -> Result<usize, RdbcError>;
    async fn delete(&mut self) -> Result<usize, RdbcError>;
    async fn get_one(&mut self) -> Result<Option<T>, RdbcError>;
    async fn insert_sensitive(&mut self) -> Result<usize, RdbcError>;
    async fn update(&mut self) -> Result<usize, RdbcError>;
    async fn update_by_query(&mut self, query: &QueryWrapper) -> Result<usize, RdbcError>;
    async fn update_sensitive(&mut self) -> Result<usize, RdbcError>;
    async fn update_sensitive_by_query(&mut self, query: &QueryWrapper)
        -> Result<usize, RdbcError>;

    async fn select_all() -> Result<Option<Vec<T>>, RdbcError>;
    async fn select_one_by_query(query: &QueryWrapper) -> Result<Option<Vec<T>>, RdbcError>;
    async fn select_list_by_query(query: &QueryWrapper) -> Result<Option<Vec<T>>, RdbcError>;
    async fn select_page_by_query(
        page_num: u32,
        page_size: u32,
        query: &QueryWrapper,
    ) -> Result<RdbcPage<T>, RdbcError>;
    async fn delete_by_query(query: &QueryWrapper) -> Result<usize, RdbcError>;
    async fn insert_batch(data: Vec<T>) -> Result<usize, RdbcError>;
}
