use super::domain_types::artifact::JsonContractSnapshotError;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = |value: &str| {
    if value.len() > constants_usize::VALUE_1_048_576 {
        Err(JsonContractSnapshotError::TooLong)
    } else {
        Ok(())
    }
})]
pub struct JsonContractSnapshot(String);
