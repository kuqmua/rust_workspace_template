#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct LeaseId(String);
impl TryFrom<String> for LeaseId {
    type Error = crate::lease_text_error::LeaseTextError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        crate::validate_lease_text::validate_lease_text(crate::lease_text_ref::LeaseTextRef::from(
            string.as_str(),
        ))
        .map(|()| Self(string))
    }
}
