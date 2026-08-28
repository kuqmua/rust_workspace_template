use crate::domain_types::InitStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::Display,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = WorkspaceMember::validate)]
pub(crate) struct WorkspaceMember(String);
impl WorkspaceMember {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), InitStringError> {
        if value.is_empty() || value.len() > 4_096usize {
            Err(InitStringError)
        } else {
            Ok(())
        }
    }
}
