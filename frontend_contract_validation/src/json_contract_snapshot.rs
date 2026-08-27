#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = JsonContractSnapshot::validate)]
pub struct JsonContractSnapshot(String);

impl JsonContractSnapshot {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), JsonContractSnapshotError> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            Err(JsonContractSnapshotError::TooLong)
        } else {
            Ok(())
        }
    }
}
pub use crate::domain_types::artifact::JsonContractSnapshotError;
