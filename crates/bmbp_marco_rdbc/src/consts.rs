pub(crate) const ATTRS_RDBC_SKIP: &str = "rdbc_skip";
pub(crate) const ATTRS_QUERY: &str = "query";
pub const RDBC_DATA_ID: &str = "data_id";
pub const RDBC_DATA_LEVEL: &str = "data_level";
pub const RDBC_DATA_STATUS: &str = "data_status";
pub const RDBC_DATA_FLAG: &str = "data_flag";
pub const RDBC_DATA_SORT: &str = "data_sort";
pub const RDBC_DATA_REMARK: &str = "data_remark";
pub const RDBC_DATA_CREATE_TIME: &str = "data_create_time";
pub const RDBC_DATA_CREATE_USER: &str = "data_create_user";
pub const RDBC_DATA_UPDATE_TIME: &str = "data_update_time";
pub const RDBC_DATA_UPDATE_USER: &str = "data_update_user";
pub const RDBC_DATA_OWNER_ORG: &str = "data_owner_org";
pub const RDBC_DATA_SIGN: &str = "data_sign";

pub const RDBC_TREE_CODE: &str = "code";
pub const RDBC_TREE_CODE_PATH: &str = "code_path";
pub const RDBC_TREE_PARENT_CODE: &str = "parent_code";
pub const RDBC_TREE_CHILDREN: &str = "children";
pub const RDBC_TREE_NAME: &str = "name";
pub const RDBC_TREE_NAME_PATH: &str = "name_path";
pub const RDBC_TREE_NODE_TYPE: &str = "node_type";
pub const RDBC_TREE_NODE_LEVEL: &str = "node_level";
pub const RDBC_TREE_NODE_GRADE: &str = "grade";

pub const RDBC_TREE_NODE_LEAF: &str = "node_leaf";

pub const RDBC_DATA_TABLE_PRIMARY_KEY: &str = bmbp_rdbc::RDBC_DATA_ID;

pub const RDBC_ENABLE: &str = "1";
pub const RDBC_DISABLE: &str = "0";
pub const RDBC_DELETE_FLAG: &str = "-1";
pub const RDBC_NEW_FLAG: &str = "0";

pub const RDBC_TREE_ROOT_NODE: &str = "0";

pub const INSERT_FIELDS: &[&str] = &[
    RDBC_DATA_ID,
    RDBC_DATA_LEVEL,
    RDBC_DATA_STATUS,
    RDBC_DATA_FLAG,
    RDBC_DATA_SORT,
    RDBC_DATA_REMARK,
    RDBC_DATA_CREATE_TIME,
    RDBC_DATA_CREATE_USER,
    RDBC_DATA_UPDATE_TIME,
    RDBC_DATA_UPDATE_USER,
    RDBC_DATA_OWNER_ORG,
    RDBC_DATA_SIGN,
];
pub const UPDATE_FIELDS: &[&str] = &[RDBC_DATA_UPDATE_TIME, RDBC_DATA_UPDATE_USER];
