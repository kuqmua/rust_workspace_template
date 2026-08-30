#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = crate::case_string_max_len::CASE_STRING_MAX_LEN)]
pub(super) struct CaseString(String);
impl CaseString {
    pub(super) fn into_inner(self) -> String {
        self.0
    }
}
