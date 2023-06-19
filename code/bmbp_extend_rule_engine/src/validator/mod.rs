use bmbp_app_common::BmbpValue;

use crate::validator::bmbp_valid_rule::BmbpValidRule;

mod bmbp_valid_rule;

#[allow(dead_code)]
pub struct ValidatorField {
    name: String,
    title: Option<String>,
    message: Option<String>,
}

impl ValidatorField {
    pub fn with_name(name: &str) -> Self {
        ValidatorField {
            name: name.to_string(),
            title: None,
            message: None,
        }
    }
    pub fn with_name_title(name: &str, title: &str) -> Self {
        ValidatorField {
            name: name.to_string(),
            title: Some(title.to_string()),
            message: None,
        }
    }

    pub fn with_name_message(name: &str, message: &str) -> Self {
        ValidatorField {
            name: name.to_string(),
            message: Some(message.to_string()),
            title: None,
        }
    }
    pub fn with_name_title_message(name: &str, title: &str, message: &str) -> Self {
        ValidatorField {
            name: name.to_string(),
            message: Some(message.to_string()),
            title: Some(title.to_string()),
        }
    }
}

#[allow(dead_code)]
pub struct ValidatorFieldRule {
    filed: ValidatorField,
    rule: Vec<BmbpValidRule>,
}

impl ValidatorFieldRule {
    pub fn custom(field: &str, custom: fn(&BmbpValue) -> bool) -> Self {
        let field = ValidatorField::with_name(field);
        let rule = BmbpValidRule::with_custom(custom);
        ValidatorFieldRule {
            filed: field,
            rule: vec![rule],
        }
    }
    pub fn custom_title(field: &str, title: &str, custom: fn(&BmbpValue) -> bool) -> Self {
        let field = ValidatorField::with_name_title(field, title);
        let rule = BmbpValidRule::with_custom(custom);
        ValidatorFieldRule {
            filed: field,
            rule: vec![rule],
        }
    }
    pub fn custom_message(field: &str, message: &str, custom: fn(&BmbpValue) -> bool) -> Self {
        let field = ValidatorField::with_name_message(field, message);
        let rule = BmbpValidRule::with_custom(custom);
        ValidatorFieldRule {
            filed: field,
            rule: vec![rule],
        }
    }

    pub fn custom_title_message(
        field: &str,
        title: &str,
        message: &str,
        custom: fn(&BmbpValue) -> bool,
    ) -> Self {
        let field = ValidatorField::with_name_title_message(field, title, message);
        let rule = BmbpValidRule::with_custom(custom);
        ValidatorFieldRule {
            filed: field,
            rule: vec![rule],
        }
    }
}
#[allow(dead_code)]
pub struct ValidatorRuleField {
    rule: BmbpValidRule,
    field: Vec<ValidatorField>,
}
#[allow(dead_code)]
pub enum ValidatorRule {
    FIELD(ValidatorFieldRule),
    RULE(ValidatorRuleField),
}

pub struct Validator {
    rules: Vec<ValidatorRule>,
}

impl Validator {
    pub fn new() -> Validator {
        Validator { rules: vec![] }
    }
    pub fn valid<T>(&self, v: T) -> Option<Vec<String>>
    where
        T: From<BmbpValue> + Into<BmbpValue>,
    {
        let value: BmbpValue = v.into();
        println!("{:#?}", value);
        None
    }
}

impl Validator {
    pub fn custom(&mut self, field: &str, custom: fn(&BmbpValue) -> bool) -> &mut Self {
        let rule = ValidatorRule::FIELD(ValidatorFieldRule::custom(field, custom));
        self.rules.push(rule);
        self
    }
    pub fn custom_title(
        &mut self,
        field: &str,
        title: &str,
        custom: fn(&BmbpValue) -> bool,
    ) -> &mut Self {
        let rule = ValidatorRule::FIELD(ValidatorFieldRule::custom_title(field, title, custom));
        self.rules.push(rule);
        self
    }

    pub fn custom_message(
        &mut self,
        field: &str,
        message: &str,
        custom: fn(&BmbpValue) -> bool,
    ) -> &mut Self {
        let rule = ValidatorRule::FIELD(ValidatorFieldRule::custom_message(field, message, custom));
        self.rules.push(rule);
        self
    }
    pub fn custom_title_message(
        &mut self,
        field: &str,
        title: &str,
        message: &str,
        custom: fn(&BmbpValue) -> bool,
    ) -> &mut Self {
        let rule = ValidatorRule::FIELD(ValidatorFieldRule::custom_title_message(
            field, title, message, custom,
        ));
        self.rules.push(rule);
        self
    }
}
