#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::BoundedStringWrapper,
    newtype::ToErrStringAsRefStr,
)]
#[bounded_string(max = crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN)]
#[serde(try_from = "String")]
pub struct LocationTestText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN },
        false,
    >,
);

impl From<&'static str> for LocationTestText {
    fn from(value: &'static str) -> Self {
        Self::try_from(String::from(value)).unwrap_or_else(Self::from)
    }
}
