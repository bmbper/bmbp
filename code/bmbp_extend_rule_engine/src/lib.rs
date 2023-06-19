pub use validator::*;

mod text;
mod validator;

#[cfg(test)]
mod tests {
    use bmbp_app_common::BmbpValue;

    use crate::Validator;

    #[test]
    fn it_works() {
        let mut validator = Validator::new();
        validator.custom("userName", |_: &BmbpValue| -> bool { false });
        validator.custom_message(
            "userName",
            "用户姓名不能为空",
            |_: &BmbpValue| -> bool { false },
        );
        validator.custom_title("userName", "用户姓名", |_: &BmbpValue| -> bool {
            false
        });

        validator.custom_title_message(
            "userName",
            "用户姓名",
            "用户姓名不能为空",
            |_: &BmbpValue| -> bool { false },
        );

        let v = BmbpValue::from("");
        let msg: Option<Vec<String>> = validator.valid::<BmbpValue>(v);
        if msg.is_none() {
            assert_eq!(true, true)
        } else {
            assert!(false)
        }
    }
}
