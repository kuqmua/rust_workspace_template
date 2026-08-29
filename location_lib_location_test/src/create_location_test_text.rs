pub(super) fn create_location_test_text(
    value: String,
) -> crate::location_test_text::LocationTestText {
    crate::location_test_text::LocationTestText::try_from(value)
        .unwrap_or_else(crate::location_test_text::LocationTestText::from)
}
