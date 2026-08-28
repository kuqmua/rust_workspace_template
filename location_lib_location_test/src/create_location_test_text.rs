use crate::domain_types::LocationTestText;

pub(super) fn create_location_test_text(value: String) -> LocationTestText {
    LocationTestText::try_from(value).unwrap_or_else(LocationTestText::from)
}
