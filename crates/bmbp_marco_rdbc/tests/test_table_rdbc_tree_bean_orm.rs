use bmbp_marco_rdbc::table_rdbc_tree_bean_orm_option;

#[test]
pub fn test_table_rdbc_tree_bean_orm_option() {
    use bmbp_rdbc_type::RdbcIdent;
    use bmbp_rdbc_type::RdbcOrmRow;
    use bmbp_rdbc_type::RdbcTable;
    use bmbp_util::BmbpTree;
    use serde::Deserialize;
    use serde::Serialize;
    #[table_rdbc_tree_bean_orm_option(table=BMBP_TABLE,tree=dict)]
    pub struct BmbpDict {
        dict_value: Option<String>,
        dict_alias: Option<String>,
    }
    let mut dict = BmbpDict::new();
    dict.set_dict_order(Some(3usize));
    assert_eq!(dict.get_dict_value().is_none(), true);
    assert_eq!(dict.get_order(), 3usize);
    println!("{:?}", dict)
}
