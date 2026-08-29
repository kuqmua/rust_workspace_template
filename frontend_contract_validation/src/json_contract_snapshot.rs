#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(error = crate::json_contract_snapshot_error::JsonContractSnapshotError, validator = |value: &str| {
    if value.len() > constants_usize::VALUE_1_048_576 {
        Err(crate::json_contract_snapshot_error::JsonContractSnapshotError::TooLong)
    } else {
        Ok(())
    }
})]
pub struct JsonContractSnapshot(String);
