#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
pub(crate) struct WorkspaceMember(String);
impl TryFrom<String> for WorkspaceMember {
    type Error = crate::init_string_error::InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 4_096usize {
            Err(Self::Error::Invalid)
        } else {
            Ok(Self(value))
        }
    }
}
