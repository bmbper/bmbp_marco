use bmbp_marco_bean::{bean, bean_option};
use serde::Deserialize;
use serde::Serialize;
#[test]
pub fn test_bean() {
    #[bean]
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Demo<T>
    where
        T: Default,
    {
        name: String,
        data: T,
    }
    let bean: Demo<String> = Demo::new();
    assert_eq!(bean.get_name().is_empty(), true);
}

#[test]
pub fn test_bean_option() {
    use serde::Deserialize;
    use serde::Serialize;
    #[bean_option]
    pub struct Demo<T>
    where
        T: Default,
    {
        name: String,
        data: Option<T>,
    }
    let bean: Demo<String> = Demo::new();
    assert_eq!(bean.get_name().is_none(), true);
}
