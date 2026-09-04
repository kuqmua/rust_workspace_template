#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct JsonContractSnapshot(String);
impl TryFrom<String> for JsonContractSnapshot {
    type Error = crate::json_contract_snapshot_error::JsonContractSnapshotError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_1_048_576)
            .then_some(Self(value))
            .ok_or(Self::Error::TooLong)
    }
}
