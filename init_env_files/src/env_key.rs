use crate::domain_types::InitStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::BorrowStr,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = EnvKey::validate)]
pub(crate) struct EnvKey(String);
impl EnvKey {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), InitStringError> {
        if value.is_empty() || value.len() > 1_024usize {
            Err(InitStringError)
        } else {
            Ok(())
        }
    }
}
