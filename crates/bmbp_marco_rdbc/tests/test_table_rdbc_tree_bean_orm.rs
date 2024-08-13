use bmbp_marco_rdbc::table_rdbc_tree_bean_orm_option;

#[test]
pub fn test_table_rdbc_tree_bean_orm_option() {
    use serde::Serialize;
    use serde::Deserialize;
    use bmbp_rdbc_type::RdbcIdent;
    use bmbp_rdbc_type::RdbcTable;
    use bmbp_rdbc_type::RdbcOrmRow;
    #[table_rdbc_tree_bean_orm_option(table=BMBP_TABLE,tree=dict)]
    pub struct BmbpDict {
        dict_value: Option<String>,
        dict_alias: Option<String>,
    }

    let dict = BmbpDict::new();
    assert_eq!(dict.get_dict_value().is_none(), true);
}
