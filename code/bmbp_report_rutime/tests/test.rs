use serde::Deserialize;
use serde::Serialize;

use bmbp_macro::{Orm, PoVo};

#[derive(PoVo, Orm)]
pub struct A {
    pub name: String,
    pub old: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BaseVoPo {
    ///  记录ID
    r_id: String,
    // 记录级別
    r_level: String,
    // 记录标记
    r_flag: String,
    // 创建时间
    r_create_time: String,
    // 创建用户
    r_create_user: String,
    // 更新时间
    r_update_time: String,
    // 更新人
    r_update_user: String,
    // 所属单位
    r_owner_org: String,
    // 所属人
    r_owner_user: String,
    // 记录签名
    r_sign: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Orm)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrganVo {
    organ_id: String,
    parent_organ_id: String,
    organ_title: String,
    organ_path: String,
    organ_data_id: String,
    organ_type: String,
    children: Vec<BmbpOrganVo>,
    #[serde(flatten)]
    base: BaseVoPo,
}

#[test]
fn test_derive_model() {
    let c = A::orm_fields();
    println!("xxx:{:#?}", c);

    let mut na = A {
        name: "X".to_string(),
        old: "Y".to_string(),
    };

    na.set_name("X1".to_string()).set_old("GG".to_string());
    assert_eq!(&"X1".to_string(), na.get_name());

    let name: &mut String = na.get_mut_name();
    name.push_str("ABC");
    assert_eq!(&"X1ABC".to_string(), na.get_name());
}

#[test]
fn test_nest_vo() {
    let _organ = BmbpOrganVo::default();
    let organ_columns = BmbpOrganVo::orm_fields();
    println!("====>{:#?}", organ_columns);
}
