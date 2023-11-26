// DDL 模块定义规范
// 数据库表索引定义
// 数据库表约束定义
// 本模块采用JSON schema 规范进行描述，仅作定义

// 数据库模式定义
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDBSchema {
    // 数据库模式编码
    code: String,
    // 数据库模式名称
    name: String,
    // 数据库模式标题
    title: String,
    // 数据库模式备注
    remark: String,
    // 数据库模式字符集
    charset: String,
    //
    owner: String,
}
impl BmbpDBSchema {
    pub fn new(code: String, name: String, title: String, remark: String, charset: String, owner: String) -> Self {
        BmbpDBSchema {
            code,
            name,
            title,
            remark,
            charset,
            owner,
        }
    }
    pub fn get_code(&self) -> String {
        self.code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_remark(&self) -> String {
        self.remark.clone()
    }
    pub fn get_charset(&self) -> String {
        self.charset.clone()
    }
    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_remark(&mut self, remark: String) {
        self.remark = remark;
    }
    pub fn set_charset(&mut self, charset: String) {
        self.charset = charset;
    }
    pub fn set_owner(&mut self, owner: String) {
        self.owner = owner;
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
    pub fn from_json_file(file_path: String) -> Self {
        let json = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
// 数据库表结构定义
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDBTable {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    code: String,
    // 数据库表名称
    name: String,
    // 数据库表标题
    title: String,
    // 数据库表备注
    remark: String,
    // 数据库表字符集
    charset: String,
    // 数据库表字段
    columns: Vec<BmbpDBColumn>,
    // 数据库表主键
    primary: Vec<BmbpDbIndex>,
    // 数据库表索引
    index: Vec<BmbpDbIndex>,
    // 数据库表约束
    constraint: Vec<BmbpDbIndex>,
}
impl BmbpDBTable {
    pub fn new(schema_code: String, code: String, name: String, title: String, remark: String, charset: String, columns: Vec<BmbpDBColumn>) -> Self {
        BmbpDBTable {
            schema_code,
            code,
            name,
            title,
            remark,
            charset,
            columns,
            primary: vec![],
            index: vec![],
            constraint: vec![],
        }
    }

    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_code(&self) -> String {
        self.code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_remark(&self) -> String {
        self.remark.clone()
    }
    pub fn get_charset(&self) -> String {
        self.charset.clone()
    }
    pub fn get_columns(&self) -> Vec<BmbpDBColumn> {
        self.columns.clone()
    }
    pub fn get_primary(&self) -> Vec<BmbpDbIndex> {
        self.primary.clone()
    }
    pub fn get_index(&self) -> Vec<BmbpDbIndex> {
        self.index.clone()
    }
    pub fn get_constraint(&self) -> Vec<BmbpDbIndex> {
        self.constraint.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_remark(&mut self, remark: String) {
        self.remark = remark;
    }
    pub fn set_charset(&mut self, charset: String) {
        self.charset = charset;
    }
    pub fn set_columns(&mut self, columns: Vec<BmbpDBColumn>) {
        self.columns = columns;
    }
    pub fn set_primary(&mut self, primary: Vec<BmbpDbIndex>) {
        self.primary = primary;
    }
    pub fn set_index(&mut self, index: Vec<BmbpDbIndex>) {
        self.index = index;
    }
    pub fn set_constraint(&mut self, constraint: Vec<BmbpDbIndex>) {
        self.constraint = constraint;
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
    pub fn from_json_file(file_path: String) -> Self {
        let json = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
    pub fn get_column_by_code(&self, code: String) -> Option<BmbpDBColumn> {
        for column in self.columns.iter() {
            if column.code == code {
                return Some(column.clone());
            }
        }
        None
    }
    pub fn get_column_by_name(&self, name: String) -> Option<BmbpDBColumn> {
        for column in self.columns.iter() {
            if column.name == name {
                return Some(column.clone());
            }
        }
        None
    }
    pub fn get_column_by_title(&self, title: String) -> Option<BmbpDBColumn> {
        for column in self.columns.iter() {
            if column.title == title {
                return Some(column.clone());
            }
        }
        None
    }
    pub fn get_column_by_remark(&self, remark: String) -> Option<BmbpDBColumn> {
        for column in self.columns.iter() {
            if column.remark == remark {
                return Some(column.clone());
            }
        }
        None
    }
    pub fn get_column_by_is_primary(&self, is_primary: bool) -> Option<BmbpDBColumn> {
        for column in self.columns.iter() {
            if column.is_primary == is_primary {
                return Some(column.clone());
            }
        }
        None
    }
}
// 数据库表字段定义
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDBColumn {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表字段编码
    code: String,
    // 数据库表字段名称
    name: String,
    // 数据库表字段标题
    title: String,
    column_type: BmbpDBColumnType,
    // 数据库表字段是否为主键
    is_primary: bool,
    // 数据库表字段是否允许为空
    is_null: bool,
    // 数据库表字段长度
    length: usize,
    // 数据库表字段精度
    scale: usize,
    // 数据库表字段默认值
    default_value: BmbpDbColumnDefaultValue,
    // 数据库表字段备注
    remark: String,
}
impl BmbpDBColumn {
    pub fn new(schema_code: String, table_code: String, code: String, name: String, title: String, column_type: BmbpDBColumnType, is_primary: bool, is_null: bool, length: usize, scale: usize, default_value: BmbpDbColumnDefaultValue, remark: String) -> Self {
        BmbpDBColumn {
            schema_code,
            table_code,
            code,
            name,
            title,
            column_type,
            is_primary,
            is_null,
            length,
            scale,
            default_value,
            remark,
        }
    }
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_code(&self) -> String {
        self.code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_column_type(&self) -> BmbpDBColumnType {
        self.column_type.clone()
    }
    pub fn get_is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn get_is_null(&self) -> bool {
        self.is_null
    }
    pub fn get_length(&self) -> usize {
        self.length
    }
    pub fn get_scale(&self) -> usize {
        self.scale
    }
    pub fn get_default_value(&self) -> BmbpDbColumnDefaultValue {
        self.default_value.clone()
    }
    pub fn get_remark(&self) -> String {
        self.remark.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_column_type(&mut self, column_type: BmbpDBColumnType) {
        self.column_type = column_type;
    }
    pub fn set_is_primary(&mut self, is_primary: bool) {
        self.is_primary = is_primary;
    }
    pub fn set_is_null(&mut self, is_null: bool) {
        self.is_null = is_null;
    }
    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }
    pub fn set_scale(&mut self, scale: usize) {
        self.scale = scale;
    }
    pub fn set_default_value(&mut self, default_value: BmbpDbColumnDefaultValue) {
        self.default_value = default_value;
    }
    pub fn set_remark(&mut self, remark: String) {
        self.remark = remark;
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
    pub fn from_json_file(file_path: String) -> Self {
        let json = std::fs::read_to_string(file_path).unwrap();
    }
}
// 数据库表字段类型定义
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum BmbpDBColumnType {
    Varchar,
    Text,
    Longtext,
    Int,
    Bigint,
    Decimal,
    Date,
    Timestamp,
    Boolean,
    Blob,
    Longblob,
}
impl BmbpDBColumnType {
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}
// 数据库表字段默认值定义
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum BmbpDbColumnDefaultValue {
    Int(i32),
    BigDecimal(f64),
    String(String),
    FUNC(String),
}
impl BmbpDbColumnDefaultValue {
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbIndex {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表索引类型
    typ: BmbpDbIndexType,
    // 数据库表索引名称
    name: String,
    // 数据库表索引列
    columns: Vec<String>,
}
impl BmbpDbIndex {
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_typ(&self) -> BmbpDbIndexType {
        self.typ.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_typ(&mut self, typ: BmbpDbIndexType) {
        self.typ = typ;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum BmbpDbIndexType {
    Primary,
    Unique,
    Index,
}
impl BmbpDbIndexType {
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}
impl BmbpDbIndex {
    pub fn new(schema_code: String, table_code: String, typ: BmbpDbIndexType, name: String, columns: Vec<String>) -> Self {
        BmbpDbIndex {
            schema_code,
            table_code,
            typ,
            name,
            columns,
        }
    }
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_typ(&self) -> BmbpDbIndexType {
        self.typ.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_typ(&mut self, typ: BmbpDbIndexType) {
        self.typ = typ;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn from_json(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
    pub fn from_json_file(file_path: String) -> Self {
        let json = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}


#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbConstraint {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表约束类型
    typ: BmbpDbConstraintType,
    // 数据库表约束名称
    name: String,
    // 数据库表约束列
    columns: Vec<String>,
}
impl BmbpDbConstraint {
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_typ(&self) -> BmbpDbConstraintType {
        self.typ.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_typ(&mut self, typ: BmbpDbConstraintType) {
        self.typ = typ;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum BmbpDbConstraintType {
    Unique,
    Check,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbView {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表视图名称
    name: String,
    // 数据库表视图SQL
    sql: String,
}
impl BmbpDbView {
    pub fn new(schema_code: String, table_code: String, name: String, sql: String) -> Self {
        BmbpDbView {
            schema_code,
            table_code,
            name,
            sql,
        }
    }
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_sql(&self) -> String {
        self.sql.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbTrigger {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表触发器名称
    name: String,
    typ: BmbpDbTriggerType,
    // 数据库表触发器SQL
    sql: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum BmbpDbTriggerType {
    Before,
    After,
}
impl BmbpDbTrigger {
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_typ(&self) -> BmbpDbTriggerType {
        self.typ.clone()
    }
    pub fn get_sql(&self) -> String {
        self.sql.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_typ(&mut self, typ: BmbpDbTriggerType) {
        self.typ = typ;
    }
    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbProcedure {
    // 数据库模式编码
    schema_code: String,
    // 数据库表编码
    table_code: String,
    // 数据库表存储过程名称
    name: String,
    // 数据库表存储过程SQL
    sql: String,
}
impl BmbpDbProcedure {
    pub fn get_schema_code(&self) -> String {
        self.schema_code.clone()
    }
    pub fn get_table_code(&self) -> String {
        self.table_code.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_sql(&self) -> String {
        self.sql.clone()
    }
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn set_schema_code(&mut self, schema_code: String) {
        self.schema_code = schema_code;
    }
    pub fn set_table_code(&mut self, table_code: String) {
        self.table_code = table_code;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct BmbpDbEvent {
    schema_code: String,
    name: String,
    sql: String,
}
impl BmbpDbEvent {
    pub fn get_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
