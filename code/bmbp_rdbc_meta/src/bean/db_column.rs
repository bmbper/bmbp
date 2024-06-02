use crate::bean::db_type::{ColumnDataType, DefaultValue};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct RdbcColumnVo {
    name: String,
    comment: Option<String>,
    data_type: ColumnDataType,
    length: Option<i32>,
    scale: Option<i32>,
    nullable: Option<bool>,
    default_value: Option<DefaultValue>,
    primary_key: Option<bool>,
    unique: Option<bool>,
}
impl RdbcColumnVo {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }
    pub fn data_type(&self) -> &ColumnDataType {
        &self.data_type
    }
    pub fn length(&self) -> Option<i32> {
        self.length
    }
    pub fn scale(&self) -> Option<i32> {
        self.scale
    }
    pub fn nullable(&self) -> Option<bool> {
        self.nullable
    }
    pub fn default_value(&self) -> &Option<DefaultValue> {
        &self.default_value
    }
    pub fn primary_key(&self) -> Option<bool> {
        self.primary_key
    }
    pub fn unique(&self) -> Option<bool> {
        self.unique
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }
    pub fn set_data_type(&mut self, data_type: ColumnDataType) {
        self.data_type = data_type;
    }
    pub fn set_length(&mut self, length: Option<i32>) {
        self.length = length;
    }
    pub fn set_scale(&mut self, scale: Option<i32>) {
        self.scale = scale;
    }
    pub fn set_nullable(&mut self, nullable: Option<bool>) {
        self.nullable = nullable;
    }
    pub fn set_default_value(&mut self, default_value: Option<DefaultValue>) {
        self.default_value = default_value;
    }
    pub fn set_primary_key(&mut self, primary_key: Option<bool>) {
        self.primary_key = primary_key;
    }
    pub fn set_unique(&mut self, unique: Option<bool>) {
        self.unique = unique;
    }
}
