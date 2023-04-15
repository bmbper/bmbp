#[test]
fn test_validator() {
    use bmbp_orm_macro::validator;
    #[validator([])]
    pub struct ValidatorDemo {}
    let demo = ValidatorDemo {};
    assert_eq!(demo.valid(), true)
}
