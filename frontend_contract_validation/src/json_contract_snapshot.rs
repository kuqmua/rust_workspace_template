#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_name_repetitions,
    reason = "the domain type and its error retain their established public names"
)]
use crate::domain_types::artifact::JsonContractSnapshotError;
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
    #[allow(
        clippy::single_call_fn,
        reason = "derive-generated TryFrom owns the single validation call"
    )]
    const fn validate(value: &str) -> Result<(), JsonContractSnapshotError> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            Err(JsonContractSnapshotError::TooLong)
        } else {
            Ok(())
        }
    }
}
