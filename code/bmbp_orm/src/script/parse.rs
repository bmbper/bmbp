use bmbp_app_common::{BmbpHashMap, BmbpValue, BmbpVec};

/// ScriptSQL 动态解析器，主要负责值的替换
/// 暂时使用正则提取标签，通过传入的实体参数中获取对映的值
///
///
const COUNT_TAG_PREFIX: char = '#';
const REPLACE_TAG_PREFIX: char = '$';
const TAG_START: char = '{';
const TAG_END: char = '}';
const ESCAPE_TAG: char = '\\';

#[derive(Debug)]
enum TokenType {
    COUNT,
    REPLACE,
}

#[allow(dead_code)]
#[derive(Debug)]
enum TokenVisitType {
    KEY,
    POSITION,
    UNION,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    _type: TokenType,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ScriptToken {
    name: String,
    _type: TokenType,
    visit_name: Vec<String>,
    visit_value: BmbpValue,
}

#[allow(dead_code)]
#[derive(Debug)]
enum TokenState {
    EMPTY = 0,
    COUNT = 1,
    REPLACE = 2,
    START = 3,
    ING = 4,
    END = 5,
}

pub struct ScriptUtil;

#[allow(unused)]
impl ScriptUtil {
    fn is_digital(value: &String) -> bool {
        return false;
    }
}

#[allow(unused)]
impl ScriptUtil {
    pub fn parse_from_map(script: &String, params: BmbpHashMap) -> (String, BmbpVec) {
        let bmbp_value = BmbpValue::Map(params);
        Self::parse(script, &bmbp_value)
    }

    pub fn parse_from_vec(script: &String, params: BmbpVec) -> (String, BmbpVec) {
        let bmbp_value = BmbpValue::Array(params);
        Self::parse(script, &bmbp_value)
    }

    pub fn parse(script: &String, params: &BmbpValue) -> (String, BmbpVec) {
        let script_params = Self::parse_script_params(script);
        let mut script_token = Self::parse_script_token(script_params.as_slice());
        Self::parse_script_token_values(script_token.as_mut_slice(), params);
        let mut values = BmbpVec::new();
        let mut sql = script.clone();
        for (idx, item) in script_token.as_slice().into_iter().enumerate() {
            sql = sql.replace(&item.name, &format!("${}", values.len() + 1));
            values.push(item.visit_value.clone());
        }
        (sql, values)
    }

    fn parse_script_token_values(script_tokens: &mut [ScriptToken], values: &BmbpValue) {
        for item in script_tokens {
            Self::parse_script_token_value(item, values);
        }
    }

    fn parse_script_token_value(script_tokens: &mut ScriptToken, values: &BmbpValue) {
        let null_value = BmbpValue::NULL;
        let mut mp_value = values;
        for token in script_tokens.visit_name.as_slice() {
            if ScriptUtil::is_digital(token) {
                match mp_value {
                    BmbpValue::Array(v) => {
                        let digital_token = 3;
                        if v.len() > (digital_token + 1) {
                            mp_value = v.get(digital_token).unwrap();
                            continue;
                        }
                    }
                    _ => {}
                }
            }
            match mp_value {
                BmbpValue::Map(mp) => {
                    if mp.contains_key(token) {
                        mp_value = mp.get(token).unwrap();
                        continue;
                    }
                }
                _ => {}
            }
            mp_value = &null_value;
            break;
        }
        script_tokens.visit_value = mp_value.clone();
    }
    /// token 提取标签
    fn parse_script_token(vars_names: &[String]) -> Vec<ScriptToken> {
        let mut token_vec = vec![];
        for item in vars_names {
            let mut token_name = {
                if item.starts_with(COUNT_TAG_PREFIX) {
                    ScriptToken {
                        name: item.clone(),
                        _type: TokenType::COUNT,
                        visit_name: vec![],
                        visit_value: BmbpValue::NULL,
                    }
                } else {
                    ScriptToken {
                        name: item.clone(),
                        _type: TokenType::REPLACE,
                        visit_name: vec![],
                        visit_value: BmbpValue::NULL,
                    }
                }
            };
            let item_slice = &item[2..(item.len() - 1)];
            let v: Vec<_> = item_slice
                .to_string()
                .split(".")
                .map(|v| v.to_string())
                .collect();
            token_name.visit_name.extend_from_slice(v.as_slice());
            token_vec.push(token_name);
        }
        token_vec
    }
    /// script 提取参数名称
    fn parse_script_params(script: &String) -> Vec<String> {
        // 参数变量
        let mut script_params: Vec<String> = vec![];

        let mut temp_params: Vec<char> = vec![];
        let mut token_state = TokenState::EMPTY;
        let mut pre_char: char = ' ';

        for char_item in script.as_bytes() {
            let item_char = (*char_item as char).clone();
            match item_char {
                ESCAPE_TAG => match token_state {
                    TokenState::EMPTY => {
                        temp_params.clear();
                    }
                    TokenState::COUNT | TokenState::REPLACE | TokenState::END => {
                        token_state = TokenState::EMPTY;
                        temp_params.clear();
                    }
                    TokenState::START | TokenState::ING => {
                        token_state = TokenState::ING;
                        temp_params.push(item_char.clone());
                    }
                },
                REPLACE_TAG_PREFIX => match token_state {
                    TokenState::EMPTY | TokenState::END => {
                        if pre_char.eq(&ESCAPE_TAG) {
                            token_state = TokenState::EMPTY;
                            temp_params.clear();
                        } else {
                            token_state = TokenState::REPLACE;
                            temp_params.push(item_char.clone());
                        }
                    }
                    TokenState::COUNT => {
                        token_state = TokenState::COUNT;
                        temp_params.clear();
                        temp_params.push(item_char.clone());
                    }
                    TokenState::REPLACE => {
                        token_state = TokenState::REPLACE;
                        temp_params.clear();
                        temp_params.push(item_char.clone());
                    }
                    TokenState::START => {
                        token_state = TokenState::START;
                        temp_params.push(item_char.clone());
                    }
                    TokenState::ING => {
                        temp_params.push(item_char.clone());
                    }
                },
                COUNT_TAG_PREFIX => match token_state {
                    TokenState::EMPTY | TokenState::END => {
                        if pre_char.eq(&ESCAPE_TAG) {
                            token_state = TokenState::EMPTY;
                            temp_params.clear();
                        } else {
                            token_state = TokenState::COUNT;
                            temp_params.push(item_char.clone());
                        }
                    }
                    TokenState::COUNT => {
                        token_state = TokenState::COUNT;
                        temp_params.clear();
                        temp_params.push(item_char.clone());
                    }
                    TokenState::REPLACE => {
                        token_state = TokenState::COUNT;
                        temp_params.clear();
                        temp_params.push(item_char.clone());
                    }
                    TokenState::START => {
                        token_state = TokenState::START;
                        temp_params.push(item_char.clone());
                    }
                    TokenState::ING => {
                        temp_params.push(item_char.clone());
                    }
                },
                TAG_START => match token_state {
                    TokenState::EMPTY | TokenState::END => {}
                    TokenState::COUNT => {
                        token_state = TokenState::START;
                        temp_params.push(item_char.clone());
                    }
                    TokenState::REPLACE => {
                        token_state = TokenState::START;
                        temp_params.push(item_char.clone());
                    }
                    TokenState::START => {
                        token_state = TokenState::START;
                        temp_params.push(item_char.clone());
                    }
                    TokenState::ING => {
                        temp_params.push(item_char.clone());
                    }
                },
                TAG_END => match token_state {
                    TokenState::EMPTY | TokenState::END => {
                        token_state = TokenState::EMPTY;
                        temp_params.clear();
                    }
                    TokenState::COUNT | TokenState::REPLACE | TokenState::START => {
                        token_state = TokenState::EMPTY;
                        temp_params.clear();
                    }
                    TokenState::ING => {
                        token_state = TokenState::EMPTY;
                        temp_params.push(item_char.clone());
                        let params_name = temp_params.iter().collect::<String>();
                        script_params.push(params_name);
                        temp_params = vec![];
                    }
                },
                _ => match token_state {
                    TokenState::EMPTY | TokenState::END => {}
                    TokenState::COUNT | TokenState::REPLACE => {
                        temp_params.clear();
                        token_state = TokenState::EMPTY;
                    }
                    TokenState::START | TokenState::ING => {
                        temp_params.push(item_char.clone());
                        token_state = TokenState::ING;
                    }
                },
            }
            pre_char = item_char.clone();
        }
        script_params
    }
}

#[cfg(test)]
mod tests {

    use bmbp_app_common::{BmbpHashMap, BmbpValue};

    use crate::script::parse::ScriptUtil;

    #[test]
    fn test_select_sql() {
        let script_sql =
            "SELECT * FROM BMBP_RBAC_ORGAN WHERE (ORGAN_ID = #{organId}) and temp = ${xxxx}"
                .to_string();

        let script_params = ScriptUtil::parse_script_params(&script_sql);
        assert_eq!(script_params.len(), 2);
        assert_eq!("#{organId}", script_params[0]);
        assert_eq!("${xxxx}", script_params[1]);

        let mut mp_values = BmbpHashMap::new();
        mp_values.insert("organId".to_string(), BmbpValue::from("004"));
        mp_values.insert("xxxx".to_string(), BmbpValue::from(33));

        let raw_sql =
            "SELECT * FROM BMBP_RBAC_ORGAN WHERE (ORGAN_ID = ${0}) and temp = ${1}".to_string();
        let (sql, value) = ScriptUtil::parse_from_map(&script_sql, mp_values);
        assert_eq!(sql, raw_sql);
        assert_eq!(BmbpValue::from("004"), value[0]);
        assert_eq!(BmbpValue::from(33), value[1]);
    }
}
