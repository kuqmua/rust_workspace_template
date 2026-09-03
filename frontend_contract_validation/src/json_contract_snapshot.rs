#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_try_from::TryFrom,
)]
#[try_from(error = crate::json_contract_snapshot_error::JsonContractSnapshotError, validator = |value: &str| {
    if value.len() > constants_usize::VALUE_1_048_576 {
        Err(crate::json_contract_snapshot_error::JsonContractSnapshotError::TooLong)
    } else {
        Ok(())
    }
})]
pub struct JsonContractSnapshot(String);
