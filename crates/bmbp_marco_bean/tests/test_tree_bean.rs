use bmbp_marco_bean::{tree_bean, tree_bean_option};
#[test]
pub fn test_bean() {
    use serde::Deserialize;
    use serde::Serialize;
    #[tree_bean(organ)]
    pub struct Demo {
        title: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.organ_name.is_none(), true);
}

#[test]
pub fn test_bean_option() {
    use serde::Deserialize;
    use serde::Serialize;
    #[tree_bean_option(organ)]
    pub struct Demo {
        title: String,
    }
    let bean = Demo::new();
    assert_eq!(bean.organ_name.is_none(), true);
}
