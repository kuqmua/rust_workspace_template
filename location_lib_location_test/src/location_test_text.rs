#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::BoundedString,
    newtype::ToErrStringAsRefStr,
)]
#[bounded_string(max = crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN)]
#[serde(try_from = "String")]
pub struct LocationTestText(String);

impl From<&'static str> for LocationTestText {
    fn from(value: &'static str) -> Self {
        Self(String::from(value))
    }
}
