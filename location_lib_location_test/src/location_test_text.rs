#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_to_err_string_as_ref_str::ToErrStringAsRefStr,
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
