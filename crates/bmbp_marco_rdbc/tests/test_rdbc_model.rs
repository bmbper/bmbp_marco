use bmbp_marco_rdbc::rdbc_model;

#[test]
fn test_rdbc_model_empty() {
    use ::serde::Deserialize;
    use ::serde::Serialize;
    use bmbp_app_common::BmbpError;
    use bmbp_app_common::BmbpPageParam;
    use bmbp_app_common::BmbpResp;
    use bmbp_app_common::HttpRespListVo;
    use bmbp_app_common::HttpRespPageVo;
    use bmbp_app_common::HttpRespVo;
    use bmbp_app_common::PageVo;
    use bmbp_app_common::RespVo;
    use bmbp_app_orm::RdbcColumn;
    use bmbp_app_orm::RdbcMacroTree;
    use bmbp_app_orm::RdbcMarcoTreeUtil;
    use bmbp_app_orm::RdbcOrmIns;
    use bmbp_app_orm::RdbcOrmRow;
    use bmbp_app_orm::RDBC_DATA_ID;
    use bmbp_app_orm::RDBC_TREE_ROOT_NODE;
    use bmbp_app_orm::{
        DeleteWrapper, InsertWrapper, QueryWrapper, RdbcFilter, RdbcTable, UpdateWrapper,
    };
    use chrono::Utc;
    use salvo::*;
    use tracing::info;
    use uuid::Uuid;
    #[rdbc_model(tree=role)]
    pub struct RdbcModel {
        #[query(eq)]
        #[valid(name(姓名),save[require(msg=""),unique(p_code),maxLength(33)])]
        name: String,
        #[valid(name("年龄"),save[require(""),uniquemaxValue(88),minValue(44),limitValue["1","3","5"]])]
        age: Option<i32>,
        #[valid(name="类型",save[require(""),limitValue["1","3","5"]])]
        grade: Option<i32>,
        #[valid(name="类型",update[require(""),unique])]
        code: Option<String>,
        #[valid(name="类型",insert[require(""),unique])]
        p_code: Option<String>,
        #[valid(name="类型",insert[require(""),unique],update[require])]
        p_code_c: Option<String>,
    }
}
