use bmbp_marco_bean::rdbc_tree_bean;
#[test]
pub fn test_rdbc_tree_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_tree_bean("org")]
    pub struct TestBean {
        name: Option<String>,
    }
    let bean = TestBean::new();
    assert_eq!(bean.get_data_id().is_none(), true);
    assert_eq!(bean.get_org_code().is_none(), true);
}
