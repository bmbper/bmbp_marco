use bmbp_marco_bean::rdbc_bean;
#[test]
pub fn test_rdbc_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_bean]
    pub struct TestBean {
        name: Option<String>,
    }
    let bean = TestBean::new();
    assert_eq!(bean.get_data_id().is_none(), true);
}
