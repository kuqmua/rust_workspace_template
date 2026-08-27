pub(in super::super) fn location_test_text(value: String) -> super::super::LocationTestText {
    super::super::LocationTestText::try_from(value)
        .unwrap_or_else(super::super::LocationTestText::from)
}
