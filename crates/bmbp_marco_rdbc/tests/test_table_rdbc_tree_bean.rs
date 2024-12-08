use bmbp_marco_rdbc::table_rdbc_tree_bean;
use bmbp_rdbc_type::RdbcTableIdent;
use bmbp_rdbc_type::RdbcIdent;
use bmbp_util::BmbpTree;
#[test]
pub fn test_table_tree_orm() {
    #[table_rdbc_tree_bean(table = BMBP_RBAC_APP_GROUP,tree=app_group)]
    #[derive(Debug, Clone, Default)]
    pub struct BmbpAppGroup {}
    let ident = BmbpAppGroupColumn::AppGroupCode.get_ident();

}
