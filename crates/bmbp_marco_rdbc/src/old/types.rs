use syn::parse::Parse;
use syn::{Expr, Field, Lit, Meta, MetaNameValue, Token};

pub struct ValidMeta {
    field: Field,
    valid: Vec<ValidRule>,
}

impl ValidMeta {
    pub fn new(field: Field, rule: Vec<ValidRule>) -> Self {
        ValidMeta { field, valid: rule }
    }
}
pub enum ValidRuleMethod {
    INSERT,
    UPDATE,
    INSERT_UPDATE,
}
#[derive(Debug, Default, Clone)]
pub struct ValidRule {
    typ: ValidRuleType,
    value: ValidRuleValue,
    msg: String,
}
#[derive(Debug, Clone)]
pub enum ValidRuleType {
    NotNull,
    Unique,
    Min,
    MinLength,
    Max,
    MaxLength,
    None,
}
impl Default for ValidRuleType {
    fn default() -> Self {
        ValidRuleType::None
    }
}
#[derive(Debug, Clone)]
pub enum ValidRuleValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    None,
}
impl Default for ValidRuleValue {
    fn default() -> Self {
        ValidRuleValue::None
    }
}
