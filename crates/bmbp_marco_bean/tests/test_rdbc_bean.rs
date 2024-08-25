use bmbp_marco_bean::{rdbc_bean, rdbc_bean_option};
#[test]
pub fn test_rdbc_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_bean]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_data_id().is_none(), true);
}
#[test]
pub fn test_rdbc_bean_option() {
    use serde::Deserialize;
    use serde::Serialize;
    #[rdbc_bean_option]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_data_id().is_none(), true);
}
