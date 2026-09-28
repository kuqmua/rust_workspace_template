#[test]
fn test_case_trait_pair_preserves_move_and_raw_parameters() {
    proc_macro_naming_common::case_trait_pair!(
        MoveRawCaseFixture,
        MoveRawTokenCaseFixture,
        std::fmt::Display,
        move |r#type| Ok(r#type.to_string())
    );
    assert_eq!(MoveRawCaseFixture::case(&42u8), 42u8.to_string());
    assert_eq!(
        MoveRawTokenCaseFixture::case_or_panic(&42u8).to_string(),
        42u8.to_string(),
    );
}
