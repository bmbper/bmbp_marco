use bmbp_marco_bean::{rdbc_tree_bean, rdbc_tree_bean_option};
#[test]
pub fn test_rdbc_tree_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_tree_bean("org")]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_data_id().is_none(), true);
    assert_eq!(bean.get_org_code().is_none(), true);
}
#[test]
pub fn test_rdbc_tree_bean_option() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_tree_bean_option("org")]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_data_id().is_none(), true);
    assert_eq!(bean.get_org_code().is_none(), true);
}
