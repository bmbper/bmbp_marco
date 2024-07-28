use bmbp_marco_bean::bean;
#[test]
pub fn test_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[bean]
    pub struct TestBean {
        name: Option<String>,
    }
    let bean = TestBean::new();
    assert_eq!(bean.get_name().is_none(), true);
}
