use bmbp_marco_bean::{bean, bean_option};
#[test]
pub fn test_bean() {
    use serde::Deserialize;
    use serde::Serialize;

    #[bean]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_name().is_empty(), true);
}

#[test]
pub fn test_bean_option() {
    use serde::Deserialize;
    use serde::Serialize;
    #[bean_option]
    pub struct Demo {
        name: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.get_name().is_none(), true);
}