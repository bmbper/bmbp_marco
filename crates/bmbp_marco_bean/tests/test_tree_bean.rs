use bmbp_marco_bean::tree_bean;
#[test]
pub fn test_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[tree_bean(organ)]
    pub struct TestTreeBean {
        name: Option<String>,
    }
    let bean = TestTreeBean::new();
    assert_eq!(bean.organ_name.is_none(), true);
}
