use bmbp_app_common::BmbpValue;

use crate::validator::bmbp_valid_rule::format::FormatRule;

pub mod format;

#[allow(dead_code)]
pub struct BmbpValidRule {
    message: Option<String>,
    ty: RuleType,
}

impl BmbpValidRule {
    pub fn with_custom(custom: fn(&BmbpValue) -> bool) -> Self {
        let rule = RuleType::Fn(custom);
        BmbpValidRule {
            message: None,
            ty: rule,
        }
    }
}

#[allow(dead_code)]
pub enum RuleType {
    NotNull,
    NotEmpty,
    Format(FormatRule),
    LENGTH((Option<usize>, Option<usize>)),
    NUMBER((Option<isize>, Option<isize>)),
    Fn(fn(&BmbpValue) -> bool),
}
