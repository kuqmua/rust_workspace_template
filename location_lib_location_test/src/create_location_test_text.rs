pub(super) fn create_location_test_text(
    string: String,
) -> crate::location_test_text::LocationTestText {
    crate::location_test_text::LocationTestText::try_from(string)
        .unwrap_or_else(crate::location_test_text::LocationTestText::from)
}
