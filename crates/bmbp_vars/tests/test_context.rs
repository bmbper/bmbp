use bmbp_vars::BMBP_CONTEXT_VARS;

#[test]
pub fn test_context() {
    BMBP_CONTEXT_VARS
        .write()
        .unwrap()
        .insert("appTitle".to_string(), "bmbp".to_string());

    assert_eq!(
        "bmbp",
        BMBP_CONTEXT_VARS
            .read()
            .unwrap()
            .get("appTitle")
            .unwrap_or(&"".to_string())
    );
}
